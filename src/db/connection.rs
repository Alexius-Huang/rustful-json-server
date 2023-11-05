use std::{
    path::PathBuf,
    fs
};
use crate::json::{field::{JsonField, ParseJsonError}, parser::read_json};

#[derive(Debug)]
pub struct Connection {
    file: PathBuf,
    json: JsonField,
    dry_run: bool
}

impl Connection {
    pub fn new(file: PathBuf) -> Result<Self, ParseJsonError> {
        let json = read_json(&file)?;

        Ok(Self { file, json, dry_run: false })
    }

    pub fn dry_run(&mut self) {
        self.dry_run = true;
    }

    // TODO: Provide option for pretty format JSON
    pub fn read(&self) -> String {
        self.json.stringify()
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
