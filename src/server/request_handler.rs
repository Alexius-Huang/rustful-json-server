use std::sync::Arc;
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
    connection: Arc<Connection>
) {
    let content = connection.read();

    let response = ResponseBuilder::new()
        .set_status_code(StatusCode::Ok)
        .set_protocol(request.version.clone())
        .set_content(content)
        .set_content_type("application/json".to_owned())
        .build();

    request.log(false);
    stream.write_all(response.format().as_bytes()).unwrap();
}

pub fn post(
    request: Request,
    mut stream: TcpStream,
    connection: Arc<Connection>
) {
    // Use this to get the request body
    let body = request.body.as_ref().unwrap();
    let json = JsonField::from(body.as_str());

    let response_body = connection.insert(json);

    let response = ResponseBuilder::new()
        .set_status_code(StatusCode::Ok)
        .set_protocol(request.version.clone())
        .set_content(response_body)
        .set_content_type("application/json".to_owned())
        .build();

    request.log(false);
    stream.write_all(response.format().as_bytes()).unwrap();
}
