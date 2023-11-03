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
    pub fn new_json_obj() -> Self {
        Self::Object(Rc::new(RefCell::new(HashMap::new())))
    }

    pub fn insert(&self, key: &str, value: Self) {
        match self {
            Self::Object(obj) => {
                obj.borrow_mut().insert(key.to_owned(), value);
            },
            _ => panic!("Unable to insert key-value pair to other than JsonField::Object variant")
        }
    }

    pub fn new_json_arr() -> Self {
        Self::Array(Rc::new(RefCell::new(vec![])))
    }

    pub fn push(&self, value: Self) {
        match self {
            Self::Array(arr) => {
                arr.borrow_mut().push(value);
            },
            _ => panic!("Unable to push value to other than JsonField::Array variant")
        }
    }

    pub fn is_null(&self) -> bool {
        self == &Self::Null
    }
}
