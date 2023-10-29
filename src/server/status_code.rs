#[derive(Debug)]
pub enum StatusCode {
    Ok,
    NotFound
}

impl StatusCode {
    pub fn get_value(&self) -> usize {
        match self {
            Self::Ok => 200,
            Self::NotFound => 404
        }
    }

    pub fn get_desc(&self) -> &str {
        match self {
            Self::Ok => "200 OK",
            Self::NotFound => "404 Not Found"
        }
    }
}

