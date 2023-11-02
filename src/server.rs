pub mod request;
pub mod response;
pub mod status_code;

use std::{
    net::{TcpListener, TcpStream},
    io::prelude::*
};

use self::response::ResponseBuilder;
use self::request::Request;
use self::status_code::StatusCode;

pub fn create(port: usize) -> TcpListener {
    let host = format!("localhost:{port}");
    println!("Listening on localhost:{port}...");

    let listener = TcpListener::bind(host).unwrap();
    listener
}

pub fn handle_connection(mut stream: TcpStream) {
    let request = Request::new(&mut stream);

    request.log();

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

    // let status_desc = "HTTP/1.1 200 OK";
    // let content_length = format!("Content-Length: {}", request_line.len());
    let response = ResponseBuilder::new()
        .set_status_code(StatusCode::Ok)
        .set_protocol(request.version)
        .set_content(r#"{ "result": "Hello World!" }"#.to_owned())
        .set_content_type("application/json".to_owned())
        .build();

    stream.write_all(response.format().as_bytes()).unwrap();    
}
