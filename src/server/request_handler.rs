use std::sync::{Arc, RwLock};
use std::net::TcpStream;
use std::io::prelude::*;

use crate::db::connection::Connection;
use crate::json::field::JsonField;
use crate::server::{
    StatusCode,
    response::ResponseBuilder,
    request::Request
};

pub fn get(
    request: Request,
    mut stream: TcpStream,
    connection: Arc<RwLock<Connection>>
) {
    let content;
    {
        let connection = connection.read().unwrap();
        content = connection.read();
    }

    let response = ResponseBuilder::new()
        .set_status_code(StatusCode::Ok)
        .set_protocol(request.version)
        .set_content(content)
        .set_content_type("application/json".to_owned())
        .build();

    stream.write_all(response.format().as_bytes()).unwrap();
}

pub fn post(
    request: Request,
    mut stream: TcpStream,
    connection: Arc<RwLock<Connection>>
) {
    // Use this to get the request body
    let body = request.body.as_ref().unwrap();
    let json = JsonField::from(body.as_str());

    let response_body;
    {
        let connection = connection.write().unwrap();
        response_body = connection.insert(json);
    }

    let response = ResponseBuilder::new()
        .set_status_code(StatusCode::Ok)
        .set_protocol(request.version)
        .set_content(response_body)
        .set_content_type("application/json".to_owned())
        .build();

    stream.write_all(response.format().as_bytes()).unwrap();
}
