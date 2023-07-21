use crate::definitions::keywords::{Snowflake, Token::*, Types};

pub fn handler(tokens: &mut Vec<Snowflake>, pos: usize, final_file: &mut Vec<String>) -> usize {
    let token = &tokens[pos].value;
    match tokens[pos - 1].value_type {
        Types::Token(Operator) => final_file.push(format!("{token}")),
        Types::Token(Keyword) => final_file.push(format!("{token}")),
        Types::Token(Comma) => final_file.push(format!("{token}")),
        Types::Token(ParenOpen) => final_file.push(format!("{token}")),
        _ => final_file.push(format!(";{token}")),
    }
    pos
}
