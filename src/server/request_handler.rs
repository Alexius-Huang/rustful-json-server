use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::{ffi::OsString, net::TcpStream};
use std::io::prelude::*;

use crate::json::field::JsonField;
use crate::server::{
    StatusCode,
    response::ResponseBuilder,
    request::Request
};
use crate::db::JsonDbConnection;

pub fn get(
    request: Request,
    mut stream: TcpStream,
    entrypoint: OsString,
    jsondb_connections: Arc<RwLock<HashMap<OsString, JsonDbConnection>>>
) {
    let content;
    {
        let connections = jsondb_connections.read().unwrap();
        let connection = connections.get(&entrypoint).unwrap();
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
    entrypoint: OsString,
    jsondb_connections: Arc<RwLock<HashMap<OsString, JsonDbConnection>>>
) {
    // Use this to get the request body
    let body = request.body.as_ref().unwrap();
    let json = JsonField::from(body.as_str());

    let response_body;
    {
        let connections = jsondb_connections.read().unwrap();
        let connection = connections.get(&entrypoint).unwrap();
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
