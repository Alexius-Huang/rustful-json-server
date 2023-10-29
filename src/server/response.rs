use super::status_code::StatusCode;

pub struct Response {
    content_length: usize,
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
        response.push_str(&new_line);
        response.push_str(&self.content);

        response
    }
}

pub struct ResponseBuilder {
    content_length: usize,
    content: String,
    protocol: String,
    status_code: StatusCode
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            content: "".to_owned(),
            content_length: 0,
            protocol: "".to_owned(),
            status_code: StatusCode::Ok
        }
    }

    pub fn set_content(mut self, content: String ) -> Self {
        self.content_length = content.len();
        self.content = content;
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
            status_code: self.status_code,
            protocol: self.protocol
        }
    }
}