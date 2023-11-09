use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;

use crate::json::field::{JsonField, ParseJsonError};
use crate::json::parser::read_json;

#[derive(Debug)]
pub struct Connection {
    file: PathBuf,
    json: JsonField,
    dry_run: bool,
    mapped: HashMap<i32, String>
}

pub struct DbQueryError<'a>(pub &'a str);

impl Connection {
    pub fn new(file: PathBuf) -> Result<Self, ParseJsonError> {
        let json = read_json(&file)?;

        let mut mapped = HashMap::new();

        {
            let arr_lock = json.unwrap_as_ref_array().unwrap_or_else(|err| {
                panic!("Reading {file:?} and the root isn't JsonField::Array type: {err:?}");
            });
    
            let arr = arr_lock.read().unwrap();
    
            for field in arr.iter() {
                let obj_lock = field.unwrap_as_ref_object();
                if obj_lock.is_err() {
                    println!("Warning: reading {file:?} and expect to get JsonField::Object type, instead got: {}", field.stringify());
                    continue;
                }
                let obj = obj_lock.unwrap().read().unwrap();

                match obj.get("id") {
                    Some(id_field) => {
                        let id = id_field.unwrap_as_ref_int();
                        if id.is_err() {
                            println!("Warning: reading {file:?} and its \"id\" field is not JsonField::Int type, instead got: {:?}", id_field.field_type());
                        } else {
                            mapped.insert(*id.unwrap(), field.stringify());
                        }
                    },
                    None => {
                        println!("Warning: reading {file:?} and it contains non relational record with content: {:?}", field.stringify());
                    }
                };
            }    
        }

        Ok(Self { file, json, dry_run: false, mapped })
    }

    pub fn dry_run(&mut self) {
        self.dry_run = true;
    }

    // TODO: Provide option for pretty format JSON
    pub fn read(&self) -> String {
        self.json.stringify()
    }

    pub fn get(&self, id: i32) -> Result<String, DbQueryError> {
        match self.mapped.get(&id) {
            Some(value) => Ok(value.clone()),
            None => Err(DbQueryError("Trying to get record with id: {id}, instead not found"))
        }
    }

    pub fn insert(&self, field: JsonField) -> String {
        // TODO: Generate Unique ID
        field.insert("id", JsonField::Int(123));
        let result = field.stringify();

        self.json.push(field);

        if !self.dry_run {
            fs::write(&self.file, self.json.stringify()).unwrap();            
        }

        result
    }
}
