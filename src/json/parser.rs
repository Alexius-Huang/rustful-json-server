mod string_parser;
mod identifier_parser;

use std::cell::RefCell;
use std::collections::HashMap;
use std::{fs, mem};
use std::path::PathBuf;
use std::rc::Rc;

use super::field::*;

pub fn read_json(path: &PathBuf) -> ParseJsonResult {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            return Err(ParseJsonError(format!("Encounter error while trying to read schema.json: {:?}", error.to_string())));
        }
    };

    parse_json(&content)
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

fn parse_json(content: &str) -> ParseJsonResult {
    let chars: Vec<char> = content.chars().collect();
    let mut result = JsonField::Null;

    let mut cur_index = 0;
    let mut cur_char = chars[cur_index];

    let len = chars.len();

    let mut layers: Vec<JsonObject> = vec![];
    let mut parsed_str_segment_cache = String::from("");
    let mut json_obj_key = String::from("");

    loop {
        match cur_char {
            '{' => {
                let json_obj: JsonObject = Rc::new(RefCell::new(HashMap::new()));
                layers.push(Rc::clone(&json_obj));

                let json_obj_field = JsonField::Object(json_obj);

                if result.is_null() {
                    result = json_obj_field;
                }
            },
            '}' => {
                match layers.pop() {
                    Some(_) => (),
                    None => {
                        return Err(ParseJsonError("Didn't have matched opening curly-brace to this closing curly-brace".to_owned()));
                    }
                }
            },
            '[' => {},
            ']' => {},
            '"' => {
                (parsed_str_segment_cache, cur_index) = string_parser::parse(cur_index, &chars)?;

                if !json_obj_key.is_empty() {
                    let cur_layer = &layers[layers.len() - 1];

                    let key = mem::take(&mut json_obj_key);
                    let value = mem::take(&mut parsed_str_segment_cache);
                    cur_layer.borrow_mut().insert(key, JsonField::String(value));
                }
            },
            ':' => {
                if layers.len() == 0 {
                    return Err(ParseJsonError(r#"Must declare JSON object using curly-braces "{{", "}}" before parsing JSON key-value"#.to_owned()));
                }

                if parsed_str_segment_cache.is_empty() {
                    return Err(ParseJsonError(r#"JSON object key should not be empty string"#.to_owned()));
                }
                mem::swap(&mut json_obj_key, &mut parsed_str_segment_cache);                
            },
            ',' => {
                if layers.len() == 0 {
                    return Err(ParseJsonError(r#"Unexpected character: ",""#.to_owned()));
                }

                let peeked_char = peek_next_non_white_space_char(cur_index, &chars);
                match peeked_char {
                    Some((peeked_char, index)) => {
                        match peeked_char {
                            '"' => {
                                cur_index = index;
                                cur_char = chars[cur_index];
                                continue;
                            },
                            _ => {
                                return Err(ParseJsonError("TODO: ADD ERROR FOR THIS TYPE".to_owned()))
                            }
                        }
                    },
                    None => {
                        return Err(ParseJsonError(r#"Unexpected character: ",""#.to_owned()));
                    }
                }
            },
            'a'..='z' => {
                if json_obj_key.is_empty() {
                    return Err(ParseJsonError(format!(r#"Unexpected character: {cur_char}"#).to_owned()));
                }

                if layers.len() == 0 {
                    return Err(ParseJsonError(r#"Must declare JSON object using curly-braces "{{", "}}" before parsing JSON key-value"#.to_owned()));
                }

                let json_field;
                (json_field, cur_index) = identifier_parser::parse(cur_index, &chars)?;

                let cur_layer = &layers[layers.len() - 1];

                let key = mem::take(&mut json_obj_key);
                cur_layer.borrow_mut().insert(key, json_field);

                cur_char = chars[cur_index];
                continue;
            },
            ' ' | '\n' => {},
            _ => {
                return Err(ParseJsonError(format!("Unexpected character: {cur_char}").to_owned()));
            }
        }

        cur_index += 1;

        if cur_index == len {
            break;
        } else {
            cur_char = chars[cur_index];
        }
    }

    if layers.len() != 0 {
        Err(ParseJsonError(r#"JSON object isn't closed properly, please close JSON object with "}""#.to_owned()))
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_json_string_into_json_fields() {
        let ex = String::from(r#"{
            "hello": "world",
            "hi": "I'm fine!",
            "is_rust": true,
            "undefined": null
        }"#);

        /*

            TODO: Support rest of the primitives
            "age": 18,
            "negative": -12,
            "float": 3.123
         */

        let mut result_obj: HashMap<String, JsonField> = HashMap::new();
        result_obj.insert("hello".to_owned(), JsonField::String("world".to_owned()));
        result_obj.insert("hi".to_owned(), JsonField::String("I'm fine!".to_owned()));
        result_obj.insert("is_rust".to_owned(), JsonField::Boolean(true));
        result_obj.insert("undefined".to_owned(), JsonField::Null);
        // result_obj.insert("age".to_owned(), JsonField::Int(18));

        let result_obj: JsonObject = Rc::new(RefCell::new(result_obj));

        assert_eq!(parse_json(&ex), Ok(JsonField::Object(result_obj)));
    }

    // TODO: Support Nested JSON parsing
    // #[test]
    // fn it_parses_nested_json_string_into_json_fields() {
    //     let ex = String::from(r#"{
    //         "parent": {
    //             "child": 123
    //         }
    //     }"#);

    //     let mut result_obj: HashMap<String, JsonField> = HashMap::new();
    //     result_obj.insert("hello".to_owned(), JsonField::String("world".to_owned()));
    //     result_obj.insert("hi".to_owned(), JsonField::String("I'm fine!".to_owned()));
    //     result_obj.insert("is_rust".to_owned(), JsonField::Boolean(true));
    //     result_obj.insert("undefined".to_owned(), JsonField::Null);

    //     let result_obj: JsonObject = Rc::new(RefCell::new(result_obj));

    //     assert_eq!(parse_json(&ex), Ok(JsonField::Object(result_obj)));
    // }

    #[test]
    fn it_returns_err_when_json_obj_without_closing_braces() {
        let ex = String::from(r#"{
            "hello": "world"
        "#);

        assert_eq!(
            parse_json(&ex),
            Err(ParseJsonError(r#"JSON object isn't closed properly, please close JSON object with "}""#.to_owned()))
        );
    }
}
