use rustful_json_server::server;

const DEFAULT_PORT: usize = 5000;

fn main() {
    let listener = server::create(DEFAULT_PORT);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        server::handle_connection(stream);
    }
}
