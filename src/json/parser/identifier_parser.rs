use crate::json::field::{JsonField, ParseJsonError};

pub fn parse(cur_index: usize, chars: &Vec<char>) -> Result<(JsonField, usize), ParseJsonError> {
    let mut ident_segment = String::new();
    let mut cur_index = cur_index;
    let mut cur_char = chars[cur_index];
    let len = chars.len();

    loop {
        ident_segment.push(cur_char);
        cur_index += 1;
        if cur_index == len {
            break Err(ParseJsonError(format!(r#"Unexpected end of JSON: {cur_char}"#).to_owned()));
        }

        cur_char = chars[cur_index];
        match cur_char {
            'a'..='z' => (),
            _ => break match ident_segment.as_str() {
                "true" => Ok((JsonField::Boolean(true), cur_index)),
                "false" => Ok((JsonField::Boolean(false), cur_index)),
                "null" => Ok((JsonField::Null, cur_index)),
                _ => Err(ParseJsonError(r#"Unexpected identifier: {ident_segment}, only these values are valid: "true", "false" or "null""#.to_owned()))
            }
        }
    }
}
