use rustful_json_server::server::Server;

const DEFAULT_PORT: usize = 5000;
const POOL_CAPACITY: usize = 4;

fn main() {
    // Clear up terminal and then position cursor at row 1 col 1
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    Server::new(DEFAULT_PORT)
        .set_pool_capacity(POOL_CAPACITY)
        .start();
}
