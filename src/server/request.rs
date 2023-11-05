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
    pub headers: HashMap<String, String>
}

pub struct RequestInitializationError(String);

impl Request {
    pub fn new(mut stream: &TcpStream) -> Self {
        let buf_reader = BufReader::new(&mut stream);
        let mut lines = buf_reader.lines();
        let mut message = lines.next().unwrap().unwrap();
    
        let request_info = message.clone();
        let mut request_info = request_info.split(" ");
        let method = RequestMethod::from(request_info.next().unwrap());

        let url_str = request_info.next().unwrap();
        let url = PathBuf::from(url_str);
        let url_string = url_str.to_owned();
        let version = request_info.next().unwrap().to_owned();
    
        let mut headers: HashMap<String, String> = HashMap::new();
        loop {
            message = lines.next().unwrap().unwrap();
            if message == "" { break; }
    
            let header = message.clone();
            let mut header = header.splitn(2, ": ");
            let header_key = header.next().unwrap();
            let header_value = header.next().unwrap();
            headers.insert(header_key.to_owned(), header_value.to_owned());
        }

        Self { method, url, url_string, version, headers }
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
