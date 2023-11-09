pub mod config;
pub mod request;
pub mod response;
pub mod status_code;
mod thread_pool;
mod request_handler;

use std::collections::HashSet;
use std::ffi::{OsString, OsStr};
use std::path::{Path, PathBuf};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::process;
use std::sync::Arc;
use std::time::Instant;

use crate::db::JsonDb;

use self::response::Response;
use self::request::{Request, RequestMethod};
use self::status_code::StatusCode;
use self::thread_pool::ThreadPool;
use self::config::Config;

pub struct Server {
    port: usize,
    listener: TcpListener,
    pool_capacity: Option<usize>,
    verbose: bool,
    dry_run: bool,
    jsondb_dir: PathBuf,
    jsondb: Option<Arc<JsonDb>>,
    main_entrypoints: Option<Arc<HashSet<OsString>>>
}

const DEFAULT_PORT: usize = 5000;
const DEFAULT_POOL_CAPACITY: usize = 4;

impl Server {
    pub fn from(config: Config) -> Self {
        let port = match config.port {
            Some(port) => port,
            None => DEFAULT_PORT
        };

        let mut server = Self::new(port, config.jsondb_dir);
        server.pool_capacity = config.pool_capacity;
        server.verbose = config.verbose;
        server.dry_run = config.dry_run;

        return server;
    }

    fn new(
        port: usize,
        jsondb_dir: PathBuf
    ) -> Self {
        let host = format!("localhost:{port}");
        let listener = TcpListener::bind(host).unwrap();

        Self {
            port,
            listener,
            pool_capacity: None,
            verbose: false,
            dry_run: false,
            jsondb_dir,
            jsondb: None,
            main_entrypoints: None
        }
    }

    pub fn start(&mut self) {
        self.jsondb = Some(Arc::new(JsonDb::new(&self.jsondb_dir, self.dry_run)));

        let mut main_entrypoints: HashSet<OsString> = HashSet::new();
        let files = fs::read_dir(self.jsondb_dir.clone()).unwrap_or_else(|err| {
            eprintln!(
                r#"Unable to read directory "{:?}": {}"#,
                self.jsondb_dir,
                err
            );
            process::exit(1);
        });

        for file in files {
            let file = file.unwrap();
            let file_name_os_str = file.file_name();
            let file_name = file_name_os_str.to_str().unwrap();
            if !file_name.ends_with(".json") || file_name == "schema.json" { continue; }

            let file_stem = Path::new(file_name).file_stem().unwrap().to_owned();

            main_entrypoints.insert(file_stem.clone());
        }

        for entrypoint in main_entrypoints.iter() {
            let entrypoint = entrypoint.to_str().unwrap();
            println!("    GET :: /{}", entrypoint);
            println!("    GET :: /{}/:id", entrypoint);
            println!("   POST :: /{}", entrypoint);
            // println!("    PUT :: /{}/:id", entrypoint);
            // println!("  PATCH :: /{}/:id", entrypoint);
            // println!(" DELETE :: /{}/:id", entrypoint);
            println!("");
        }

        self.main_entrypoints = Some(Arc::new(main_entrypoints));

        let pool_capacity = if self.pool_capacity.is_some() {
            self.pool_capacity.unwrap()
        } else {
            DEFAULT_POOL_CAPACITY
        };

        let pool = ThreadPool::new(pool_capacity);
        let main_entrypoints = self.main_entrypoints.as_ref().unwrap();

        println!("Listening on localhost:{}...", self.port);
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let main_entrypoints = Arc::clone(main_entrypoints);
            let jsondb = Arc::clone(self.jsondb.as_ref().unwrap());

            pool.execute(move || Self::handle_connection(
                stream,
                main_entrypoints,
                jsondb
            ));
        }    
    }

    fn handle_connection(
        mut stream: TcpStream,
        main_entrypoints: Arc<HashSet<OsString>>,
        jsondb: Arc<JsonDb>
    ) {
        let now = Instant::now();
        let request = Request::new(&mut stream, now);
        if request.is_err() {
            return Response::not_found("HTTP/1.1".to_owned(), now, stream)
        }
        let request = request.unwrap();
        let version = request.version.clone();

        let path_segments: Vec<&OsStr> = request.url.iter().collect();
        if path_segments.len() < 2 {
            return Response::not_found(version, now, stream);
        }

        let entrypoint = path_segments[1].to_owned();
        if !main_entrypoints.contains(&entrypoint) {
            return Response::not_found(version, now, stream);
        }

        let connection = Arc::clone(&jsondb.get_entry(entrypoint));

        /* Get all or insert new record */
        if path_segments.len() == 2 {
            match request.method {
                RequestMethod::GET => request_handler::get(
                    request,
                    stream,
                    connection
                ),
                RequestMethod::POST => request_handler::post(
                    request,
                    stream,
                    connection
                ),
                _ => Response::not_found(version, now, stream)
            }
            return;
        }

        /* Get specific record */
        if path_segments.len() == 3 {
            let id: i32 = match path_segments[2].to_os_string().into_string() {
                Ok(string) => match string.parse() {
                    Ok(value) => value,
                    Err(_) => return Response::not_found(version, now, stream)
                },
                Err(_) => return Response::not_found(version, now, stream)
            };

            match request.method {
                RequestMethod::GET => request_handler::get_id(
                    request,
                    stream,
                    connection,
                    id
                ),
                _ => Response::not_found(version, now, stream)
            }
            return;
        }
    }
}
