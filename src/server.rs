pub mod config;
pub mod request;
pub mod response;
pub mod status_code;
pub mod thread_pool;

use std::{
    net::{TcpListener, TcpStream},
    io::prelude::*
};

use self::response::ResponseBuilder;
use self::request::Request;
use self::status_code::StatusCode;
use self::thread_pool::ThreadPool;
use self::config::Config;

pub struct Server {
    listener: TcpListener,
    pool_capacity: Option<usize>,
    verbose: bool
}

const DEFAULT_PORT: usize = 5000;
const DEFAULT_POOL_CAPACITY: usize = 4;

impl Server {
    pub fn from(config: Config) -> Self {
        let port = match config.port {
            Some(port) => port,
            None => DEFAULT_PORT
        };

        let mut server = Self::new(port);
        server.pool_capacity = config.pool_capacity;
        server.verbose = config.verbose;

        return server;
    }

    pub fn new(port: usize) -> Self {
        let host = format!("localhost:{port}");
        println!("Listening on localhost:{port}...");
    
        let listener = TcpListener::bind(host).unwrap();
        Self { listener, pool_capacity: None, verbose: false }
    }

    pub fn start(&self) {
        let pool_capacity = if self.pool_capacity.is_some() {
            self.pool_capacity.unwrap()
        } else {
            DEFAULT_POOL_CAPACITY
        };

        let pool = ThreadPool::new(pool_capacity);

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let verbose = self.verbose;

            pool.execute(move || {
                Self::handle_connection(stream, verbose);
            });
        }    
    }

    pub fn handle_connection(mut stream: TcpStream, verbose: bool) {
        let request = Request::new(&mut stream);

        // TODO: Create logging queue for proper logging
        request.log(verbose);
    
        // let (status_desc, file_path) = match request_line.as_str() {
        //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            // "GET /sleep HTTP/1.1" => {
            //     /* Simulating Slow Response... */
            //     thread::sleep(Duration::from_secs(5));
            //     ("HTTP/1.1 200 OK", "hello.html")
            // },
        //     _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
        // };
    
        // let (content_length, contents) = read_content(file_path);
    
        let response = ResponseBuilder::new()
            .set_status_code(StatusCode::Ok)
            .set_protocol(request.version)
            .set_content(r#"{ "result": "Hello World!" }"#.to_owned())
            .set_content_type("application/json".to_owned())
            .build();
    
        stream.write_all(response.format().as_bytes()).unwrap();    
    }    
}
