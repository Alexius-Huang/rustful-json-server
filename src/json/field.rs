use std::{
    collections::{HashMap, HashSet},
    sync::RwLock
};

use core::convert::From;
use super::parser;

pub type JsonObject = HashMap<String, JsonField>;
pub type WrappedJsonObject = RwLock<JsonObject>;
pub type JsonArray = Vec<JsonField>;
pub type WrappedJsonArray = RwLock<JsonArray>;

#[derive(Debug)]
pub enum JsonField {
    Int(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Object(WrappedJsonObject),
    Array(WrappedJsonArray),
    Null
}

impl From<&str> for JsonField {
    fn from(content: &str) -> Self {
        parser::parse_json(content, 0).unwrap().0
    }
}

// RwLock doesn't have PartialEq trait
impl PartialEq for JsonField {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Null, Self::Null) => true,
            (Self::Object(a), Self::Object(b)) => {
                let read_a = a.read().unwrap();
                let read_b = b.read().unwrap();
                
                let keys_a: HashSet<&String> = read_a.iter().map(|(k, _)| k).collect();
                let mut keys_b: HashSet<&String> = HashSet::new();
                for (key_b, _) in read_b.iter() {
                    if !keys_a.contains(key_b) { return false; }
                    keys_b.insert(key_b);
                }

                if keys_a.len() != keys_b.len() { return false; }

                keys_a.iter().all(|&key| {
                    let value_a = read_a.get(key).unwrap();
                    let value_b = read_b.get(key).unwrap();
                    value_a == value_b
                })
            },
            (Self::Array(a), Self::Array(b)) => {
                let read_a = a.read().unwrap();
                let read_b = b.read().unwrap();

                if read_a.len() != read_b.len() { return false; }

                read_a.iter().enumerate().all(|(index, field)| {
                    field == &read_b[index]
                })
            },
            _ => false
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ParseJsonError(pub String);

pub type ParseJsonResult = Result<(JsonField, usize), ParseJsonError>;

impl JsonField {
    pub fn new_json_obj() -> Self {
        Self::Object(RwLock::new(HashMap::new()))
    }

    pub fn insert(&self, key: &str, value: Self) {
        match self {
            Self::Object(obj) => {
                obj.write().unwrap().insert(key.to_owned(), value);
            },
            _ => panic!("Unable to insert key-value pair to other than JsonField::Object variant")
        }
    }

    pub fn new_json_arr() -> Self {
        Self::Array(RwLock::new(vec![]))
    }

    pub fn push(&self, value: Self) {
        match self {
            Self::Array(arr) => {
                arr.write().unwrap().push(value);
            },
            _ => panic!("Unable to push value to other than JsonField::Array variant")
        }
    }

    pub fn is_null(&self) -> bool {
        self == &Self::Null
    }
}
