use super::field::JsonField;

impl JsonField {
    pub fn stringify(&self) -> String {
        match self {
            Self::Null => "null".to_owned(),
            Self::Boolean(true) => "true".to_owned(),
            Self::Boolean(false) => "false".to_owned(),
            Self::Int(value) => format!("{}", value),
            Self::Float(value) => format!("{}", value),
            Self::String(value) => format!(r#""{value}""#),
            Self::Array(rw_lock) => {
                let arr = rw_lock.read().unwrap();
                let mut result = "[".to_owned();

                let len = arr.len();
                for i in 0..(len - 1) {
                    let field = &arr[i];
                    result.push_str(&field.stringify());
                    result.push(',');
                }

                result.push_str(&arr[len - 1].stringify());
                result.push(']');
                result
            },
            Self::Object(rw_lock) => {
                let obj = rw_lock.read().unwrap();
                let mut result = "{".to_owned();

                let pairs: Vec<(&String, &JsonField)> = obj.iter().collect();
                let len = pairs.len();

                for i in 0..(len - 1) {
                    let (key, field) = pairs[i];
                    result.push('"');
                    result.push_str(key);
                    result.push_str("\":");
                    result.push_str(&field.stringify());
                    result.push(',');
                }

                let (key, field) = pairs[len - 1];
                result.push('"');
                result.push_str(key);
                result.push_str("\":");
                result.push_str(&field.stringify());
                result.push('}');
                result
            }
        }
    }
}
