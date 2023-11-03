use crate::json::field::{JsonField, ParseJsonError};

pub fn parse(cur_index: &mut usize, chars: &Vec<char>) -> Result<JsonField, ParseJsonError> {
    let mut num_str = String::new();
    let len = chars.len();

    loop {
        num_str.push(chars[*cur_index]);
        *cur_index += 1;

        if *cur_index == len {
            return Err(ParseJsonError(format!(r#"Unexpected end of JSON with number: {num_str}"#).to_owned()));
        }

        let cur_char = chars[*cur_index];
        match cur_char {
            '0'..='9' => continue,
            _ => {
                let num: i32 = num_str.parse().unwrap();
                break Ok(JsonField::Int(num));
            }
        }
    }
}
