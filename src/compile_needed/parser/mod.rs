use crate::definitions::keywords::{Snowflake, Token::*, Types};

mod handlers;
use handlers::*;

pub fn run(tokens: &mut Vec<Snowflake>) -> String {
    let mut final_file = vec!["#[allow(unused)] fn main() {".to_string()];
    let mut pos = 0;
    while pos < tokens.len() {
        pos = token_handler(tokens, pos, &mut final_file) + 1;
    }
    return final_file.join("\n") + "}\n";
}

fn token_handler(tokens: &mut Vec<Snowflake>, pos: usize, final_file: &mut Vec<String>) -> usize {
    match tokens[pos].value_type {
        Types::String => {
            final_file.push(format!("\"{}\"", &tokens[pos].value));
            pos
        }
        Types::Token(Keyword) => match tokens[pos].value.as_str() {
            "let" => let_k::handler(pos, final_file),
            "const" => const_k::handler(pos, final_file),
            "use" => use_k::handler(tokens, pos),
            "export" => export_k::handler(tokens, pos, final_file),
            _ => unreachable!(),
        },
        Types::Token(Word) => word::handler(tokens, pos, final_file),
        _ => general_handler(tokens, final_file, pos),
    }
}

fn general_handler(tokens: &mut Vec<Snowflake>, final_file: &mut Vec<String>, pos: usize) -> usize {
    let token = &tokens[pos].value;
    final_file.push(token.to_owned());
    pos
}
