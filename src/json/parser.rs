mod string_parser;
mod identifier_parser;
mod number_parser;

use std::{fs, mem};
use std::path::PathBuf;

use super::field::*;

pub fn read_json(path: &PathBuf) -> ParseJsonResult {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            return Err(ParseJsonError(format!("Encounter error while trying to read schema.json: {:?}", error.to_string())));
        }
    };

    parse_json(&content, 0)
}

fn peek_next_non_white_space_char(start_index: usize, chars: &Vec<char>) -> Option<(char, usize)> {
    let mut index = start_index + 1;

    while index < chars.len() {
        match chars[index] {
            ' ' | '\n' => { index += 1; },
            _ => { return Some((chars[index], index)); }
        }
    }

    None
}

fn parse_json(content: &str, starting_index: usize) -> ParseJsonResult {
    let chars: Vec<char> = content.chars().collect();
    let mut result = JsonField::Null;

    let mut cur_index = starting_index;
    let mut cur_char = chars[cur_index];

    let len = chars.len();

    let mut json_obj_key = String::from("");

    loop {
        match cur_char {
            '{' => {
                if result.is_null() {
                    let json_obj = JsonField::new_json_obj();
                    let json_obj_field = JsonField::Object(json_obj);
                    result = json_obj_field;
                } else if json_obj_key.is_empty() {
                    return Err(ParseJsonError(r#"JSON object key should not be empty string"#.to_owned()));
                } else {
                    let child_obj;
                    (child_obj, cur_index) = parse_json(content, cur_index)?;

                    let key = mem::take(&mut json_obj_key);
                    match result {
                        JsonField::Object(ref obj) => {
                            obj.borrow_mut().insert(key, child_obj);
                        },
                        _ => {
                            return Err(ParseJsonError("TODO: Figure out the error message here".to_owned()));
                        }
                    }
                }
            },
            '}' => return match result {
                JsonField::Object(field) =>Ok((JsonField::Object(field), cur_index)),
                _ => Err(ParseJsonError("Didn't have matched opening curly-brace to this closing curly-brace".to_owned()))
            },
            '[' => {
                if result.is_null() {
                    let json_arr: WrappedJsonArray = JsonField::new_json_arr();
                    let json_arr_field = JsonField::Array(json_arr);
                    result = json_arr_field;
                } else {
                    return Err(ParseJsonError("TODO: Not handled recursive array case yet!".to_owned()));
                }
            },
            ']' => return match result {
                JsonField::Array(arr) => Ok((JsonField::Array(arr), cur_index)),
                _ => Err(ParseJsonError("Didn't have matched opening bracket to this closing bracket".to_owned()))
            },
            '"' => match result {
                JsonField::Object(ref obj) => {
                    if json_obj_key.is_empty() {
                        json_obj_key = string_parser::parse(&mut cur_index, &chars)?;
                        cur_index += 1;
                        if chars[cur_index] != ':' {
                            return Err(ParseJsonError(r#"Expect to have ":" right after JSON object key"#.to_owned()));
                        }
                    } else {
                        let key = mem::take(&mut json_obj_key);
                        let value = string_parser::parse(&mut cur_index, &chars)?;  
                        obj.borrow_mut().insert(key, JsonField::String(value));
                    }
                },
                JsonField::Array(ref obj) => {
                    let value = string_parser::parse(&mut cur_index, &chars)?;  
                    obj.borrow_mut().push(JsonField::String(value));
                },
                _ => return Err(ParseJsonError("TODO: Explain this error!".to_owned()))
            },
            ',' => match result {
                JsonField::Object(_) => {
                    let peeked_char = peek_next_non_white_space_char(cur_index, &chars);
                    if peeked_char.is_none() {
                        return Err(ParseJsonError(r#"Unexpected character: ",""#.to_owned()));
                    }
    
                    let (peeked_char, index) = peeked_char.unwrap();
                    if peeked_char == '"' {
                        cur_index = index;
                        cur_char = chars[cur_index];
                        continue;
                    } else {
                        return Err(ParseJsonError("JSON object's key must be double quoted string".to_owned()))
                    }
                },
                JsonField::Array(_) => (),
                _ => return Err(ParseJsonError(r#"Unexpected character: ",""#.to_owned()))
            },
            '-' | '0'..='9' => {
                match result {
                    JsonField::Object(ref obj) => {
                        if json_obj_key.is_empty() {
                            return Err(ParseJsonError(format!(r#"Unexpected character: "{cur_char}""#).to_owned()));
                        }
                        let json_field = number_parser::parse(&mut cur_index, &chars)?;
                        obj.borrow_mut().insert(mem::take(&mut json_obj_key), json_field);
                    },
                    JsonField::Array(ref arr) => {
                        let json_field = number_parser::parse(&mut cur_index, &chars)?;
                        arr.borrow_mut().push(json_field);
                    },
                    _ => return Err(ParseJsonError(r#"Unexpected character: ",""#.to_owned()))
                }
                cur_char = chars[cur_index];
                continue;
            },
            'a'..='z' => {
                if json_obj_key.is_empty() {
                    return Err(ParseJsonError(format!(r#"Unexpected character: "{cur_char}""#).to_owned()));
                }

                if result.is_null() {
                    return Err(ParseJsonError(r#"Must declare JSON object using curly-braces "{{", "}}" before parsing JSON key-value"#.to_owned()));
                }

                let json_field = identifier_parser::parse(&mut cur_index, &chars)?;
                let key = mem::take(&mut json_obj_key);
                match result {
                    JsonField::Object(ref obj) => {
                        obj.borrow_mut().insert(key, json_field);
                    },
                    _ => {
                        return Err(ParseJsonError("TODO: Explain this error!".to_owned()));
                    }
                }

                cur_char = chars[cur_index];
                continue;
            },
            ' ' | '\n' => {},
            _ => return Err(ParseJsonError(format!(r#"Unexpected character: "{cur_char}""#).to_owned()))
        }

        cur_index += 1;

        if cur_index == len {
            break;
        } else {
            cur_char = chars[cur_index];
        }
    }

    Err(ParseJsonError(r#"Unexpected end of JSON, couldn't parse correctly, perhaps missing closing braces "}" or bracket "]""#.to_owned()))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    
    #[test]
    fn it_parses_json_string_into_json_fields() {
        let ex = String::from(r#"{
            "hello": "world",
            "hi": "I'm fine!",
            "is_rust": true,
            "undefined": null,
            "age": 18,
            "something-else": "123",
            "negative": -12,
            "float": 0.123,
            "neg-float": -9.876
        }"#);

        let mut result_obj: JsonObject = HashMap::new();
        result_obj.insert("hello".to_owned(), JsonField::String("world".to_owned()));
        result_obj.insert("hi".to_owned(), JsonField::String("I'm fine!".to_owned()));
        result_obj.insert("is_rust".to_owned(), JsonField::Boolean(true));
        result_obj.insert("undefined".to_owned(), JsonField::Null);
        result_obj.insert("age".to_owned(), JsonField::Int(18));
        result_obj.insert("something-else".to_owned(), JsonField::String("123".to_owned()));
        result_obj.insert("negative".to_owned(), JsonField::Int(-12));
        result_obj.insert("float".to_owned(), JsonField::Float(0.123));
        result_obj.insert("neg-float".to_owned(), JsonField::Float(-9.876));

        let result_obj: WrappedJsonObject = Rc::new(RefCell::new(result_obj));

        assert_eq!(parse_json(&ex, 0), Ok((JsonField::Object(result_obj), ex.len() - 1)));
    }

    #[test]
    fn it_parses_nested_json_string_into_json_fields() {
        let ex = String::from(r#"{
            "parent": {
                "child": 123
            },
            "prop-in-parent": true,
            "parent-2": {
                "child": "this is nested",
                "child-2": {
                    "grand-child": 0.123
                }
            }
        }"#);

        let mut child_obj: JsonObject = HashMap::new();
        child_obj.insert("child".to_owned(), JsonField::Int(123));
        let child_obj: WrappedJsonObject = Rc::new(RefCell::new(child_obj));

        let mut grandchild_obj: JsonObject = HashMap::new();
        grandchild_obj.insert("grand-child".to_owned(), JsonField::Float(0.123));
        let grandchild_obj: WrappedJsonObject = Rc::new(RefCell::new(grandchild_obj));

        let mut child_obj2: JsonObject = HashMap::new();
        child_obj2.insert("child".to_owned(), JsonField::String("this is nested".to_owned()));
        child_obj2.insert("child-2".to_owned(), JsonField::Object(grandchild_obj));
        let child_obj2: WrappedJsonObject = Rc::new(RefCell::new(child_obj2));

        let mut result_obj: JsonObject = HashMap::new();
        result_obj.insert("parent".to_owned(), JsonField::Object(child_obj));
        result_obj.insert("prop-in-parent".to_owned(), JsonField::Boolean(true));
        result_obj.insert("parent-2".to_owned(), JsonField::Object(child_obj2));

        let result_obj: WrappedJsonObject = Rc::new(RefCell::new(result_obj));

        assert_eq!(parse_json(&ex, 0), Ok((JsonField::Object(result_obj), ex.len() - 1)));
    }

    #[test]
    fn it_parses_array_of_elements() {
        let ex = String::from(r#"[
            -987.456
            "Hello World",
            123,
            "Hi!"
        ]"#);

        assert_eq!(
            parse_json(&ex, 0),
            Ok((
                JsonField::Array(Rc::new(RefCell::new(vec![
                    JsonField::Float(-987.456),
                    JsonField::String("Hello World".to_owned()),
                    JsonField::Int(123),
                    JsonField::String("Hi!".to_owned())
                ]))),
                ex.len() - 1
            ))
        )
    }

    #[test]
    fn it_returns_err_when_json_obj_without_closing_braces() {
        let ex = String::from(r#"{
            "hello": "world"
        "#);

        assert_eq!(
            parse_json(&ex, 0),
            Err(ParseJsonError(r#"Unexpected end of JSON, couldn't parse correctly, perhaps missing closing braces "}" or bracket "]""#.to_owned()))
        );
    }

    #[test]
    fn it_does_not_allow_key_without_double_quotes_surrounded() {
        let ex = String::from(r#"{
            hello: "world"
        "#);

        assert_eq!(
            parse_json(&ex, 0),
            Err(ParseJsonError(r#"Unexpected character: "h""#.to_owned()))
        );

        let ex = String::from(r#"{
            "hello": "world",
            hi: "How are you?"
        "#);

        assert_eq!(
            parse_json(&ex, 0),
            Err(ParseJsonError(r#"JSON object's key must be double quoted string"#.to_owned()))
        );
    }
}
