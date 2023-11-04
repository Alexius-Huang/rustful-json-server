mod string_parser;
mod identifier_parser;
mod number_parser;

use std::{fs, mem};
use std::path::PathBuf;

use super::field::*;

pub fn read_json(path: &PathBuf) -> Result<JsonField, ParseJsonError> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            return Err(ParseJsonError(format!("Encounter error while trying to read schema.json: {:?}", error.to_string())));
        }
    };

    let (data, _) = parse_json(&content, 0)?;
    Ok(data)
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
                match result {
                    JsonField::Null => {
                        result = JsonField::new_json_obj();
                    },
                    JsonField::Object(ref obj) => {
                        if json_obj_key.is_empty() {
                            return Err(ParseJsonError(r#"JSON object key should not be empty string"#.to_owned()));
                        }
                        let child_obj;
                        (child_obj, cur_index) = parse_json(content, cur_index)?;
                        obj.write().unwrap().insert(mem::take(&mut json_obj_key), child_obj);
                    },
                    JsonField::Array(ref arr) => {
                        let child_obj;
                        (child_obj, cur_index) = parse_json(content, cur_index)?;
                        arr.write().unwrap().push(child_obj);
                    },
                    _ => panic!("Should never reach here")
                }
            },
            '}' => return match result {
                JsonField::Object(field) => Ok((JsonField::Object(field), cur_index)),
                _ => Err(ParseJsonError("Didn't have matched opening curly-brace to this closing curly-brace".to_owned()))
            },
            '[' => {
                match result {
                    JsonField::Null => {
                        result = JsonField::new_json_arr();
                    },
                    JsonField::Object(ref obj) => {
                        if json_obj_key.is_empty() {
                            return Err(ParseJsonError(r#"JSON object key should not be empty string"#.to_owned()));
                        }
                        let json_arr;
                        (json_arr, cur_index) = parse_json(content, cur_index)?;
                        obj.write().unwrap().insert(mem::take(&mut json_obj_key), json_arr);
                    },
                    _ => panic!("Should never reach here!")
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
                        obj.write().unwrap().insert(key, JsonField::String(value));
                    }
                },
                JsonField::Array(ref obj) => {
                    let value = string_parser::parse(&mut cur_index, &chars)?;  
                    obj.write().unwrap().push(JsonField::String(value));
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
                    }
                    return Err(ParseJsonError("JSON object's key must be double quoted string".to_owned()))
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
                        obj.write().unwrap().insert(mem::take(&mut json_obj_key), json_field);
                    },
                    JsonField::Array(ref arr) => {
                        let json_field = number_parser::parse(&mut cur_index, &chars)?;
                        arr.write().unwrap().push(json_field);
                    },
                    _ => return Err(ParseJsonError(r#"Unexpected character: ",""#.to_owned()))
                }
                cur_char = chars[cur_index];
                continue;
            },
            'a'..='z' => {
                match result {
                    JsonField::Object(ref obj) => {
                        if json_obj_key.is_empty() {
                            return Err(ParseJsonError(format!(r#"Unexpected character: "{cur_char}""#).to_owned()));
                        }
                        let json_field = identifier_parser::parse(&mut cur_index, &chars)?;
                        obj.write().unwrap().insert(mem::take(&mut json_obj_key), json_field);
                    },
                    JsonField::Array(ref arr) => {
                        let json_field = identifier_parser::parse(&mut cur_index, &chars)?;
                        arr.write().unwrap().push(json_field);
                    },
                    _ => return Err(ParseJsonError(r#"Unexpected character: ",""#.to_owned()))
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

        let result_obj = JsonField::new_json_obj();
        result_obj.insert("hello", JsonField::String("world".to_owned()));
        result_obj.insert("hi", JsonField::String("I'm fine!".to_owned()));
        result_obj.insert("is_rust", JsonField::Boolean(true));
        result_obj.insert("undefined", JsonField::Null);
        result_obj.insert("age", JsonField::Int(18));
        result_obj.insert("something-else", JsonField::String("123".to_owned()));
        result_obj.insert("negative", JsonField::Int(-12));
        result_obj.insert("float", JsonField::Float(0.123));
        result_obj.insert("neg-float", JsonField::Float(-9.876));

        assert_eq!(parse_json(&ex, 0), Ok((result_obj, ex.len() - 1)));
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

        let child_obj = JsonField::new_json_obj();
        child_obj.insert("child", JsonField::Int(123));

        let grandchild_obj = JsonField::new_json_obj();
        grandchild_obj.insert("grand-child", JsonField::Float(0.123));

        let child_obj2 = JsonField::new_json_obj();
        child_obj2.insert("child", JsonField::String("this is nested".to_owned()));
        child_obj2.insert("child-2", grandchild_obj);

        let result_obj= JsonField::new_json_obj();
        result_obj.insert("parent", child_obj);
        result_obj.insert("prop-in-parent", JsonField::Boolean(true));
        result_obj.insert("parent-2", child_obj2);

        assert_eq!(parse_json(&ex, 0), Ok((result_obj, ex.len() - 1)));
    }

    #[test]
    fn it_parses_array_of_elements() {
        let ex = String::from(r#"[
            -987.456,
            null,
            "Hello World",
            false,
            123,
            "Hi!"
        ]"#);

        let result = JsonField::new_json_arr();
        result.push(JsonField::Float(-987.456));
        result.push(JsonField::Null);
        result.push(JsonField::String("Hello World".to_owned()));
        result.push(JsonField::Boolean(false));
        result.push(JsonField::Int(123));
        result.push(JsonField::String("Hi!".to_owned()));

        assert_eq!(
            parse_json(&ex, 0),
            Ok((result, ex.len() - 1))
        );
    }

    #[test]
    fn it_parses_object_containing_array() {
        let ex = String::from(r#"{
            "numbers": [1, 2, 3]
        }"#);

        let result = JsonField::new_json_obj();
        let arr = JsonField::new_json_arr();
        arr.push(JsonField::Int(1));
        arr.push(JsonField::Int(2));
        arr.push(JsonField::Int(3));
        result.insert("numbers", arr);

        assert_eq!(parse_json(&ex, 0), Ok((result, ex.len() - 1)));
    }

    #[test]
    fn it_parses_array_containing_object() {
        let ex = String::from(r#"[
            { "prop": 123 }
        ]"#);

        let obj = JsonField::new_json_obj();
        obj.insert("prop", JsonField::Int(123));

        let result = JsonField::new_json_arr();
        result.push(obj);

        assert_eq!(parse_json(&ex, 0), Ok((result, ex.len() - 1)));
    }

    #[test]
    fn it_parses_composite_object() {
        let ex = String::from(r#"{
            "prop": "something",
            "arr": [
                123,
                { "child": true },
                null
            ]
        }"#);

        let arr = JsonField::new_json_arr();
        arr.push(JsonField::Int(123));

        let child = JsonField::new_json_obj();
        child.insert("child", JsonField::Boolean(true));

        arr.push(child);
        arr.push(JsonField::Null);

        let result = JsonField::new_json_obj();
        result.insert("prop", JsonField::String("something".to_owned()));
        result.insert("arr", arr);

        assert_eq!(parse_json(&ex, 0), Ok((result, ex.len() - 1)));
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
