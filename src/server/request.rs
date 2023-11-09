use std::{
    collections::HashMap,
    net::TcpStream,
    io::{prelude::*, BufReader},
    path::PathBuf,
    convert::From, time::Instant
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
        match value.to_ascii_uppercase().as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            _ => Self::GET
            // _ => panic!("{}", format!("Failed to initialize RequstMethod from \"{}\"", value))
        }
    }
}

pub struct Request {
    pub start_time: Instant,
    pub method: RequestMethod,
    pub url: PathBuf,
    url_string: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

pub struct RequestInitializationError(String);

impl Request {
    pub fn new(mut stream: &TcpStream, start_time: Instant) -> Result<Self, String> {        
        let mut buf_reader = BufReader::new(&mut stream);
        let mut request_info = String::new();
        buf_reader.read_line(&mut request_info).unwrap();

        // TODO: Figure out a way to handle errors!
        let mut request_info = request_info.trim_end().split(" ");
        let method = request_info.next();
        if method.is_none() {
            return Err("Empty Request Method".to_owned());
        }
        let method = RequestMethod::from(method.unwrap());

        let url_str = request_info.next();
        if url_str.is_none() {
            return Err("Empty Request URL".to_owned());
        }
        let url_str = url_str.unwrap();

        let url = PathBuf::from(url_str);
        let url_string = url_str.to_owned();
        let version = request_info.next().unwrap().trim_end().to_owned();

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut message = String::new();
        loop {
            let size = buf_reader.read_line(&mut message).unwrap();
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

        Ok(Self {
            method,
            url,
            url_string,
            version,
            headers,
            body,
            start_time
        })
    }

    pub fn log(&self, verbose: bool) {
        let duration = Instant::now() - self.start_time;
        println!("{:?} :: {} {:?}", self.method, self.url_string, duration);

        if verbose {
            for (key, value) in self.headers.iter() {
                println!("{key}: {value}");
            }
        }
    }
}
