use rustful_json_server::{server, thread_pool};

const DEFAULT_PORT: usize = 5000;
const POOL_CAPACITY: usize = 4;

fn main() {
    // Clear up terminal and then position cursor at row 1 col 1
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
    let listener = server::create(DEFAULT_PORT);
    let pool = thread_pool::ThreadPool::new(POOL_CAPACITY);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            server::handle_connection(stream);
        });
    }
}
