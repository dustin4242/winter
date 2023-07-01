use crate::definitions::keywords::{Snowflake, Token::*, Types};
use std::collections::HashMap;

pub fn hunt(tokens: &Vec<Snowflake>) {
    let mut variables: HashMap<String, Types> = HashMap::new();
    let mut functions: HashMap<String, Vec<Types>> = HashMap::new();
    let mut pos = 0;
    while pos < tokens.len() {
        token_handler(tokens, pos, &mut functions, &mut variables);
        pos += 1;
    }
}

fn token_handler(
    tokens: &Vec<Snowflake>,
    pos: usize,
    functions: &mut HashMap<String, Vec<Types>>,
    variables: &mut HashMap<String, Types>,
) {
    match tokens[pos].value_type {
        Types::Token(Keyword) => match tokens[pos].value.as_str() {
            "use" => use_handler(tokens, pos, functions),
            "let" => let_handler(tokens, pos, variables),
            _ => (),
        },
        Types::Token(Word) => word_handler(pos, tokens, variables, functions),
        _ => (),
    }
}

fn let_handler(tokens: &Vec<Snowflake>, pos: usize, variables: &mut HashMap<String, Types>) {
    if pos + 4 > tokens.len() {
        if pos + 2 > tokens.len() {
            panic!("Missing 2 descriptors for let statement");
        }
    }
    if tokens[pos + 1].value_type != Types::Token(Word) {
        panic!(
            "Expected a word but found a {}",
            tokens[pos + 1].value_type.to_string()
        );
    }
    if tokens[pos + 2].value_type == Types::Token(TypeAssignment) {
        if tokens[pos + 3].value_type != Types::Token(Word) {
            panic!("\"{}\" is not a valid type", tokens[pos + 3].value);
        }
        variables.insert(
            tokens[pos + 1].value.clone(),
            Types::to_type(tokens[pos + 2].value.to_owned()),
        );
    } else {
        variables.insert(tokens[pos + 1].value.clone(), tokens[pos + 2].value_type);
    }
}

fn use_handler(tokens: &Vec<Snowflake>, pos: usize, functions: &mut HashMap<String, Vec<Types>>) {
    if pos + 1 > tokens.len() {
        panic!("No snow file specified to use");
    }
    if tokens[pos + 1].value_type != Types::String {
        panic!(
            "{} is not a valid use descriptor",
            tokens[pos + 1].value_type.to_string()
        )
    }
    functions.insert(tokens[pos + 1].value.clone(), vec![]);
}

fn word_handler(
    pos: usize,
    tokens: &Vec<Snowflake>,
    variables: &HashMap<String, Types>,
    functions: &HashMap<String, Vec<Types>>,
) {
    let contains = (
        variables.contains_key(&tokens[pos].value),
        functions.contains_key(&tokens[pos].value),
        Types::is_type(tokens[pos].value.to_owned()),
    );
    if !contains.0 && !contains.1 && !contains.2 {
        if pos.checked_sub(1).is_some()
            && tokens[pos - 1].value_type == Types::Token(TypeAssignment)
        {
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
                    tokens[pos + i].value_type.to_string(),
                    args[i].to_string()
                );
            }
        }
    }
}
