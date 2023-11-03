use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

pub type JsonObject = HashMap<String, JsonField>;
pub type WrappedJsonObject = Rc<RefCell<JsonObject>>;
pub type JsonArray = Vec<JsonField>;
pub type WrappedJsonArray = Rc<RefCell<JsonArray>>;

#[derive(PartialEq, Debug)]
pub enum JsonField {
    Int(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Object(WrappedJsonObject),
    Array(WrappedJsonArray),
    Null
}

#[derive(PartialEq, Debug)]
pub struct ParseJsonError(pub String);

pub type ParseJsonResult = Result<(JsonField, usize), ParseJsonError>;

impl JsonField {
    pub fn new_json_obj() -> WrappedJsonObject {
        Rc::new(RefCell::new(HashMap::new()))
    }

    pub fn new_json_arr() -> WrappedJsonArray {
        Rc::new(RefCell::new(vec![]))
    }

    pub fn is_null(&self) -> bool {
        self == &Self::Null
    }
}
