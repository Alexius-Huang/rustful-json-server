use crate::json::field::ParseJsonError;

pub fn parse(cur_index: &mut usize, chars: &Vec<char>) -> Result<String, ParseJsonError> {
    let mut str_segment = String::new();
    let len = chars.len();

    loop {
        *cur_index += 1;
        if *cur_index == len {
            break Err(ParseJsonError(r#"Expect to close of string with another closing '"' character"#.to_owned()));
        }
        let cur_char = chars[*cur_index];
        if cur_char == '"' {
            break Ok(str_segment);
        }

        str_segment.push(cur_char);
    }
}
