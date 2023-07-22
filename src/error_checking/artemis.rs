use crate::definitions::keywords::{Snowflake, Token::*, Types};
use std::collections::HashMap;

mod handlers;
use handlers::*;

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
            "let" => let_k::handler(tokens, pos, variables),
            "const" => const_k::handler(tokens, pos, variables),
            "use" => use_k::handler(tokens, pos, functions),
            _ => (),
        },
        Types::Token(Word) => word::handler(pos, tokens, variables, functions),
        _ => (),
    }
}
