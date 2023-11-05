pub mod connection;

use std::{
    path::{PathBuf, Path},
    fs,
    collections::HashMap,
    ffi::OsString,
    sync::{Arc, RwLock}
};
use self::connection::Connection;

pub struct JsonDb {
    connections: HashMap<OsString, Arc<RwLock<Connection>>>
}

impl JsonDb {
    pub fn new(root_dir: &PathBuf, dry_run: bool) -> Self {
        let files = fs::read_dir(root_dir.clone()).unwrap();
        let mut connections = HashMap::new();

        println!("=========== Reading JSON ===========");
        for file in files {  
            let file_name_os_str = file.unwrap().file_name();
            let file_name = file_name_os_str.to_str().unwrap();
            if !file_name.ends_with(".json") || file_name == "schema.json" { continue; }
    
            let file_stem = Path::new(file_name).file_stem().unwrap().to_owned();

            println!("Connecting ... {}", file_name);

            let file_path = root_dir.join(file_name);
            let mut connection = Connection::new(file_path).unwrap();

            if dry_run { connection.dry_run(); }

            connections.insert(file_stem, Arc::new(RwLock::new(connection)));
        }
        println!("");

        Self { connections }
    }

    pub fn get_entry(&self, entrypoint: OsString) -> Arc<RwLock<Connection>> {
        Arc::clone(self.connections.get(&entrypoint).unwrap())
    }
}

