use crate::tokenizer::Snowflake;
use std::{collections::HashMap, io::Error};

pub fn artemis(tokens: &Vec<Snowflake>) -> Result<(), Error> {
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut functions: HashMap<String, Vec<String>> = HashMap::new();
    let mut pos = 0;
    while pos < tokens.len() {
        match tokens[pos].value_type.as_str() {
            "keyword" => keyword_handler(pos, tokens, &mut variables, &mut functions),
            "word" => word_handler(pos, tokens, &variables, &functions),
            _ => (),
        }
        pos += 1;
    }
    Ok(())
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
            if pos + 2 > tokens.len() {
                panic!("Missing 2 descriptors for let statement")
            }
            if tokens[pos + 1].value_type != "word".to_string() {
                panic!("Expected a word but found a {}", tokens[pos + 1].value_type);
            }
            variables.insert(tokens[pos + 1].value.clone(), tokens[pos + 2].value.clone());
        }
        _ => (),
    }
}

fn word_handler(
    pos: usize,
    tokens: &Vec<Snowflake>,
    variables: &HashMap<String, String>,
    functions: &HashMap<String, Vec<String>>,
) {
    let contains = (
        variables.contains_key(&tokens[pos].value),
        functions.contains_key(&tokens[pos].value),
    );
    if !contains.0 && !contains.1 {
        panic!("Unknown variable used: {}", tokens[pos].value);
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
