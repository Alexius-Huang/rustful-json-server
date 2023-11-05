use std::{
    collections::HashMap,
    net::TcpStream,
    io::{prelude::*, BufReader},
    path::PathBuf,
    convert::From
};

#[derive(Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

impl From<&str> for RequestMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            _ => panic!(
                "Failed to initialize RequstMethod from string"
            )
        }
    }
}

pub struct Request {
    pub method: RequestMethod,
    pub url: PathBuf,
    url_string: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

pub struct RequestInitializationError(String);

impl Request {
    pub fn new(mut stream: &TcpStream) -> Self {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut request_info = String::new();
        buf_reader.read_line(&mut request_info).unwrap();

        let mut request_info = request_info.split(" ");
        let method = RequestMethod::from(request_info.next().unwrap());

        let url_str = request_info.next().unwrap();
        let url = PathBuf::from(url_str);
        let url_string = url_str.to_owned();
        let version = request_info.next().unwrap().trim_end().to_owned();

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut message = String::new();
        loop {
            let size = buf_reader.read_line(&mut message).unwrap();// lines.next().unwrap().unwrap();
            if size < 3 { break; }

            let header = message.clone();
            let mut header = header.splitn(2, ": ");
            let header_key = header.next().unwrap();
            let header_value = header.next().unwrap().trim_end();
            headers.insert(header_key.to_owned(), header_value.to_owned());

            message.clear();
        }

        let mut body: Option<String> = None;
        if let Some(content_len) = headers.get("Content-Length") {
            let content_len = content_len.parse::<usize>().unwrap();
            let mut buffer = vec![0; content_len];
            buf_reader.read_exact(&mut buffer).unwrap();
            body = Some(String::from_utf8(buffer).unwrap());
        }

        Self { method, url, url_string, version, headers, body }
    }

    pub fn log(&self, verbose: bool) {
        println!("{:?} :: {}", self.method, self.url_string);

        if verbose {
            for (key, value) in self.headers.iter() {
                println!("{key}: {value}");
            }
        }
    }
}
