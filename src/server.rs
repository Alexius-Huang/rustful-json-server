pub mod config;
pub mod request;
pub mod response;
pub mod status_code;
mod thread_pool;
mod request_handler;

use std::collections::{HashSet, HashMap};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::net::{TcpListener, TcpStream};
use std::fs::{self, DirEntry};
use std::process;
use std::sync::{Arc, RwLock};

use crate::db::JsonDbConnection;

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
    jsondb_dir: PathBuf,
    jsondb_connections: Option<Arc<RwLock<HashMap<OsString, JsonDbConnection>>>>,
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
            jsondb_dir,
            jsondb_connections: None,
            main_entrypoints: None
        }
    }

    pub fn start(&mut self) {
        self.setup_db_connections();

        let pool_capacity = if self.pool_capacity.is_some() {
            self.pool_capacity.unwrap()
        } else {
            DEFAULT_POOL_CAPACITY
        };

        let pool = ThreadPool::new(pool_capacity);
        let main_entrypoints = self.main_entrypoints.as_ref().unwrap();
        let jsondb_connections = self.jsondb_connections.as_ref().unwrap();

        println!("Listening on localhost:{}...", self.port);
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let verbose = self.verbose;
            let main_entrypoints = Arc::clone(main_entrypoints);
            let jsondb_connections = Arc::clone(jsondb_connections);

            pool.execute(move || Self::handle_connection(
                stream,
                verbose,
                main_entrypoints,
                jsondb_connections
            ));
        }    
    }

    fn setup_db_connections(&mut self) {
        let files = fs::read_dir(self.jsondb_dir.clone()).unwrap_or_else(|err| {
            eprintln!(
                r#"Unable to read directory "{:?}": {}"#,
                self.jsondb_dir,
                err
            );
            process::exit(1);
        });

        let mut file_dir_entries: Vec<DirEntry> = vec![];
        let mut main_entrypoints: HashSet<OsString> = HashSet::new();
        let mut jsondb_connections: HashMap<OsString, JsonDbConnection> = HashMap::new();

        println!("=========== Reading JSON ===========");
        for file in files {
            let file = file.unwrap_or_else(|err| {
                eprintln!(
                    r#"Trying to read files in directory "{:?}", however encountered error: {}"#,
                    self.jsondb_dir,
                    err
                );
                process::exit(1);
            });
    
            let file_name_os_str = file.file_name();
            let file_name = file_name_os_str.to_str().unwrap();
            if !file_name.ends_with(".json") || file_name == "schema.json" { continue; }
    
            file_dir_entries.push(file);
            let file_stem = Path::new(file_name).file_stem().unwrap().to_owned();

            main_entrypoints.insert(file_stem.clone());

            println!("Connecting ... {}", file_name);

            let file_path = self.jsondb_dir.clone().join(file_name);
            let connection = JsonDbConnection::new(file_path).unwrap_or_else(|err| {
                eprintln!(
                    r#"Encounter error while creating JSON database connection: {:?}"#,
                    err
                );
                process::exit(1);
            });
            jsondb_connections.insert(file_stem, connection);
        }
        println!("");

        println!("====== Available Entry Points ======");

        println!("");
        for entrypoint in main_entrypoints.iter() {
            let entrypoint = entrypoint.to_str().unwrap();
            println!("    GET :: /{}", entrypoint);
            // println!("   POST :: /{}", entrypoint);
            // println!("    PUT :: /{}/:id", entrypoint);
            // println!("  PATCH :: /{}/:id", entrypoint);
            // println!(" DELETE :: /{}/:id", entrypoint);
            println!("");
        }

        self.main_entrypoints = Some(Arc::new(main_entrypoints));
        self.jsondb_connections = Some(Arc::new(RwLock::new(jsondb_connections)));
    }

    fn handle_connection(
        mut stream: TcpStream,
        verbose: bool,
        main_entrypoints: Arc<HashSet<OsString>>,
        jsondb_connections: Arc<RwLock<HashMap<OsString, JsonDbConnection>>>
    ) {
        let request = Request::new(&mut stream);

        // TODO: Create logging queue for proper logging
        request.log(verbose);
    
        let mut path_segment = request.url.iter();
        path_segment.next();

        let entrypoint = path_segment.next();
        if entrypoint.is_none() {
            return Response::not_found(request, stream);
        }

        let entrypoint = entrypoint.unwrap().to_owned();
        if !main_entrypoints.contains(&entrypoint) {
            return Response::not_found(request, stream);
        }

        match request.method {
            RequestMethod::GET => request_handler::get(
                request,
                stream,
                entrypoint,
                jsondb_connections
            ),
            RequestMethod::POST => request_handler::post(
                request,
                stream,
                entrypoint,
                jsondb_connections
            ),
            _ => return Response::not_found(request, stream)
        }
    }
}
