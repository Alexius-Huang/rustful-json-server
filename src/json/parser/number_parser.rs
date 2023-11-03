use crate::json::field::{JsonField, ParseJsonError};

pub fn parse(cur_index: &mut usize, chars: &Vec<char>) -> Result<JsonField, ParseJsonError> {
    let mut num_str = String::new();
    let mut is_float = false;
    let len = chars.len();

    loop {
        num_str.push(chars[*cur_index]);
        *cur_index += 1;

        if *cur_index == len {
            return Err(ParseJsonError(format!(r#"Unexpected end of JSON with number: {num_str}"#).to_owned()));
        }

        match chars[*cur_index] {
            '0'..='9' => (),
            '.' => {
                if is_float {
                    return Err(ParseJsonError(format!(r#"Trying to parse number, encountered recurring dot while parsing floating point number "{}""# , num_str).to_owned()));
                }
                is_float = true;
            },
            _ => {
                if is_float {
                    let num: f64 = num_str.parse().unwrap();
                    break Ok(JsonField::Float(num));    
                }
                
                let num: i32 = num_str.parse().unwrap();
                break Ok(JsonField::Int(num));    
            }
        }
    }
}
