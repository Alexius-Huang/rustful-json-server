use crate::json::field::ParseJsonError;

pub fn parse(cur_index: usize, chars: &Vec<char>) -> Result<(String, usize), ParseJsonError> {
    let mut cur_index = cur_index;
    let len = chars.len();
    let mut str_segment = String::new();

    loop {
        cur_index += 1;
        if cur_index == len {
            break Err(ParseJsonError(r#"Expect to close of string with another closing '"' character"#.to_owned()));
        }
        let cur_char = chars[cur_index];
        if cur_char == '"' {
            break Ok((str_segment, cur_index));
        }

        str_segment.push(cur_char);
    }
}
