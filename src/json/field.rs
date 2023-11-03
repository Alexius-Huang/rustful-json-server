use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

pub type JsonObject = Rc<RefCell<HashMap<String, JsonField>>>;

#[derive(PartialEq, Debug)]
pub enum JsonField {
    Int(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Object(JsonObject),
    // Array(Box<Vec<Field>>),
    Null
}

#[derive(PartialEq, Debug)]
pub struct ParseJsonError(pub String);

pub type ParseJsonResult = Result<JsonField, ParseJsonError>;

impl JsonField {
    pub fn is_null(&self) -> bool {
        self == &Self::Null
    }
}
