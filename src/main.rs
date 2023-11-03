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

    let mut has_schema_json = false;
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
        if !file_name.ends_with(".json") { continue; }

        if file_name == "schema.json" {
            has_schema_json = true;
        } else {
            file_dir_entries.push(file);
        }
        println!("Processing ... {}", file_name);
    }

    if !has_schema_json {
        eprintln!(
            "Cannot found schema.json, expect to have schema.json which serves as JSON database schema"
        );
        process::exit(1);
    }

    println!("");

    println!("TODO:");
    println!("- Refactor and also start read JSON data");
    println!("- Implement test");

    println!("");
    Server::from(config).start();
}
