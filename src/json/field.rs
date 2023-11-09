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
    Array(RwLock<JsonArray>),
    Null
}

#[derive(PartialEq, Debug)]
pub enum JsonFieldType {
    Int,
    Float,
    String,
    Boolean,
    Object,
    Array,
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
    pub fn is(&self, field_type: JsonFieldType) -> bool {
        match &self {
            Self::Int(_) if field_type == JsonFieldType::Int => true,
            Self::Float(_) if field_type == JsonFieldType::Float => true,
            Self::Boolean(_) if field_type == JsonFieldType::Boolean => true,
            Self::String(_) if field_type == JsonFieldType::String => true,
            Self::Array(_) if field_type == JsonFieldType::Array => true,
            Self::Object(_) if field_type == JsonFieldType::Object => true,
            Self::Null if field_type == JsonFieldType::Null => true,
            _ => false
        }
    }

    pub fn field_type(&self) -> JsonFieldType {
        match &self {
            Self::Int(_) => JsonFieldType::Int,
            Self::Float(_) => JsonFieldType::Float,
            Self::Boolean(_) => JsonFieldType::Boolean,
            Self::String(_) => JsonFieldType::String,
            Self::Array(_) => JsonFieldType::Array,
            Self::Object(_) => JsonFieldType::Object,
            Self::Null => JsonFieldType::Null,
        }
    }

    pub fn unwrap_as_ref_array(&self) -> Result<&RwLock<JsonArray>, ParseJsonError> {
        match self {
            JsonField::Array(arr_lock) => return Ok(arr_lock),
            _ => Err(ParseJsonError(format!("Expect to unwrap as JsonField::Array type, instead got: {:?}", self.field_type())))
        }
    }

    pub fn unwrap_as_ref_object(&self) -> Result<&RwLock<JsonObject>, ParseJsonError> {
        match self {
            JsonField::Object(obj_lock) => return Ok(obj_lock),
            _ => Err(ParseJsonError(format!("Expect to unwrap as JsonField::Object type, instead got: {:?}", self.field_type())))
        }
    }

    pub fn unwrap_as_ref_int(&self) -> Result<&i32, ParseJsonError> {
        match self {
            JsonField::Int(value) => return Ok(value),
            _ => Err(ParseJsonError(format!("Expect to unwrap as JsonField::Int type, instead got: {:?}", self.field_type())))
        }
    }

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
