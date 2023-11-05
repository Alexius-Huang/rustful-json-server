use std::net::TcpStream;
use std::io::prelude::*;

use super::status_code::StatusCode;
use super::request::Request;

pub struct Response {
    content_length: usize,
    content_type: Option<String>,
    content: String,
    protocol: String,
    status_code: StatusCode
}

impl Response {
    pub fn format(&self) -> String {
        let new_line: String = String::from("\r\n");

        let mut response = format!("{} {}", self.protocol, self.status_code.get_desc());
        response.push_str(&new_line);
        response.push_str(&format!("Content-Length: {}", self.content_length));
        response.push_str(&new_line);
        if self.content_type.is_some() {
            response.push_str(&format!("Content-Type: {}", self.content_type.as_ref().unwrap()));
            response.push_str(&new_line);
        }
        response.push_str(&new_line);
        response.push_str(&self.content);

        response
    }

    pub fn not_found(version: String, mut stream: TcpStream) {
        let response = ResponseBuilder::build_404(version);
        stream.write_all(response.format().as_bytes()).unwrap();
    }
}

pub struct ResponseBuilder {
    content_length: usize,
    content: String,
    protocol: String,
    status_code: StatusCode,
    content_type: Option<String>
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            content: "".to_owned(),
            content_length: 0,
            content_type: None,
            protocol: "".to_owned(),
            status_code: StatusCode::Ok
        }
    }

    pub fn set_content(mut self, content: String ) -> Self {
        self.content_length = content.len();
        self.content = content;
        self
    }

    pub fn set_content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn set_protocol(mut self, protocol: String) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn set_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn build(self) -> Response {
        Response {
            content: self.content,
            content_length: self.content_length,
            content_type: self.content_type,
            status_code: self.status_code,
            protocol: self.protocol
        }
    }

    pub fn build_404(version: String) -> Response {
        Self::new()
            .set_status_code(StatusCode::NotFound)
            .set_protocol(version)
            .set_content(r#"{ "message": "404 Not Found" }"#.to_owned())
            .set_content_type("application/json".to_owned())
            .build()
    }
}
