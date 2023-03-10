use crate::{hail, tokenizer::Snowflake};
use std::collections::HashMap;

pub fn hunt(tokens: &Vec<Snowflake>) {
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut functions: HashMap<String, Vec<String>> = HashMap::new();
    let mut pos = 0;
    let types = hail::types();
    while pos < tokens.len() {
        match tokens[pos].value_type.as_str() {
            "keyword" => keyword_handler(pos, tokens, &mut variables, &mut functions),
            "word" => word_handler(pos, tokens, &types, &variables, &functions),
            _ => (),
        }
        pos += 1;
    }
}

fn keyword_handler(
    pos: usize,
    tokens: &Vec<Snowflake>,
    variables: &mut HashMap<String, String>,
    functions: &mut HashMap<String, Vec<String>>,
) {
    match tokens[pos].value.as_str() {
        "use" => {
            if pos + 1 > tokens.len() {
                panic!("No snow file specified to use");
            }
            if tokens[pos + 1].value_type != "string".to_string() {
                panic!(
                    "{} is not a valid use descriptor",
                    tokens[pos + 1].value_type
                )
            }
            functions.insert(tokens[pos + 1].value.clone(), vec![]);
        }
        "let" => {
            if pos + 4 > tokens.len() {
                if pos + 2 > tokens.len() {
                    panic!("Missing 2 descriptors for let statement");
                }
            }
            if tokens[pos + 1].value_type != "word".to_string() {
                panic!("Expected a word but found a {}", tokens[pos + 1].value_type);
            }
            if tokens[pos + 2].value_type == "type_assignment".to_string() {
                if tokens[pos + 3].value_type != "word" {
                    panic!("\"{}\" is not a valid type", tokens[pos + 3].value);
                }
                variables.insert(tokens[pos + 1].value.clone(), tokens[pos + 2].value.clone());
            } else {
                variables.insert(
                    tokens[pos + 1].value.clone(),
                    tokens[pos + 2].value_type.clone(),
                );
            }
        }
        _ => (),
    }
}

fn word_handler(
    pos: usize,
    tokens: &Vec<Snowflake>,
    types: &Vec<String>,
    variables: &HashMap<String, String>,
    functions: &HashMap<String, Vec<String>>,
) {
    let contains = (
        variables.contains_key(&tokens[pos].value),
        functions.contains_key(&tokens[pos].value),
        types.contains(&tokens[pos].value),
    );
    if !contains.0 && !contains.1 && !contains.2 {
        if pos.checked_sub(1).is_some() && &tokens[pos - 1].value_type == "type_assignment" {
            panic!("Unknown type used: {}", tokens[pos].value);
        } else {
            panic!("Unknown variable used: {}", tokens[pos].value);
        }
    } else if contains.1 {
        let args = functions.get(&tokens[pos].value).unwrap();
        for i in 0..args.len() {
            if tokens[pos + i].value_type != args[i] {
                panic!(
                    "Types do not match: {} compared to {}",
                    tokens[pos + i].value_type,
                    args[i]
                );
            }
        }
    }
}
