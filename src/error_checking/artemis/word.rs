use crate::definitions::keywords::{Snowflake, Token::*, Types};
use std::collections::HashMap;

pub fn handler(
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
