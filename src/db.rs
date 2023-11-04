use std::path::PathBuf;
use crate::json::{field::{JsonField, ParseJsonError}, parser::read_json};

#[derive(Debug)]
pub struct JsonDbConnection {
    file: PathBuf,
    json: JsonField
}

impl JsonDbConnection {
    pub fn new(file: PathBuf) -> Result<Self, ParseJsonError> {
        let json = read_json(&file)?;

        Ok(Self { file, json })
    }
}
