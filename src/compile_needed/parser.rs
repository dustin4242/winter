use crate::definitions::keywords::{Snowflake, Token::*, Types};

mod handlers;
use handlers::*;

pub fn run(tokens: &mut Vec<Snowflake>) -> String {
    let mut final_file = vec!["#[allow(unused)] fn main() {".to_string()];
    let mut pos = 0;
    while pos < tokens.len() {
        pos = token_handler(tokens, pos, &mut final_file) + 1;
    }
    return final_file.join("\n") + ";}\n";
}

fn token_handler(tokens: &mut Vec<Snowflake>, pos: usize, final_file: &mut Vec<String>) -> usize {
    match tokens[pos].value_type {
        Types::Token(Keyword) => match tokens[pos].value.as_str() {
            "let" => let_keyword::handler(pos, final_file),
            "const" => const_keyword::handler(pos, final_file),
            "use" => use_keyword::handler(tokens, pos),
            "export" => export_keyword::handler(tokens, pos, final_file),
            "write" => write_keyword::handler(tokens, pos, final_file),
            _ => unreachable!(),
        },
        Types::Token(Word) => word::handler(tokens, pos, final_file),
        _ => passthrough(tokens, final_file, pos),
    }
}

fn passthrough(tokens: &mut Vec<Snowflake>, final_file: &mut Vec<String>, pos: usize) -> usize {
    let token = &tokens[pos].value;
    final_file.push(token.to_owned());
    pos
}
