use std::env;
use std::process;
use rustful_json_server::server::{
    Server,
    config::Config
};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::from(args).unwrap_or_else(|err| {
        eprintln!("Parsing config error: {err}");
        process::exit(1)
    });

    // Clear up terminal and then position cursor at row 1 col 1
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    Server::from(config).start();
}
