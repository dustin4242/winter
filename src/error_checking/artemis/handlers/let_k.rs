use crate::definitions::keywords::{Snowflake, Token::*, Types};
use std::collections::HashMap;

pub fn handler(tokens: &Vec<Snowflake>, pos: usize, variables: &mut HashMap<String, Types>) {
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
