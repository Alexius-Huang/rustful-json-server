use std::env;
use std::fs;
use std::fs::DirEntry;
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
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let files = fs::read_dir(&config.jsondb_dir).unwrap_or_else(|err| {
        eprintln!(
            r#"Unable to read directory "{:?}": {}"#,
            &config.jsondb_dir,
            err
        );
        process::exit(1);
    });

    let schema_file_path = config.jsondb_dir.join("schema.json");
    let schema_file = match fs::read_to_string(schema_file_path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Encounter error while trying to read schema.json: {:?}", error.to_string());
            println!("Encounter error while trying to read schema.json: {:?}", error.to_string());
            process::exit(1);
        }
    };

    // TODO: Parse Schema File, which means implementing JSON parser
    println!("{schema_file:?}");

    let mut file_dir_entries: Vec<DirEntry> = vec![];
    for file in files {
        let file = file.unwrap_or_else(|err| {
            eprintln!(
                r#"Trying to read files in directory "{:?}", however encountered error: {}"#,
                &config.jsondb_dir,
                err
            );
            process::exit(1);
        });

        let file_name_os_str = file.file_name();
        let file_name = file_name_os_str.to_str().unwrap();
        if !file_name.ends_with(".json") || file_name == "schema.json" { continue; }

        file_dir_entries.push(file);
        println!("Processing ... {}", file_name);
    }

    println!("");

    println!("TODO:");
    println!("- Refactor and also start read JSON data");
    println!("- Implement test");

    println!("");
    Server::from(config).start();
}
