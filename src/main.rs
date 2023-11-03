use std::env;
use std::fs;
use std::fs::DirEntry;
use std::process;
// use rustful_json_server::json::parser::read_json;
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

    // let schema_file_path = config.jsondb_dir.join("schema.json");
    // let schema_json = read_json(&schema_file_path).unwrap_or_else(|err| {
    //     eprintln!(
    //         r#"Unable to read "{:?}": {:?}"#,
    //         &schema_file_path,
    //         err
    //     );
    //     process::exit(1);        
    // });
    // println!("Reading schema.json");
    // println!("{:?}", schema_json);

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

        // let file_path = config.jsondb_dir.join(file_name);
        // let json = read_json(&file_path).unwrap_or_else(|err| {
        //     eprintln!(
        //         r#"Unable to read "{:?}": {:?}"#,
        //         &file_path,
        //         err
        //     );
        //     process::exit(1);        
        // });
        // println!("Reading {}", file_name);
        // println!("{:?}", json);    
    }

    println!("");

    println!("TODO:");
    println!("- Refactor and also start read JSON data");
    println!("- Implement test");

    println!("");
    Server::from(config).start();
}
