pub mod response;
pub mod status_code;

use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}
};

use self::response::ResponseBuilder;
use self::status_code::StatusCode;

pub fn create(port: usize) -> TcpListener {
    let host = format!("localhost:{port}");
    let stream = TcpListener::bind(host).unwrap();
    stream
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // TODO: Handle requests
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // let new_line = "\r\n";

    // println!("RECEIVED REQUEST!");

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
        .set_protocol("HTTP/1.1".to_owned())
        .set_content("Hello World!".to_owned())
        .build();

    stream.write_all(response.format().as_bytes()).unwrap();    
}
