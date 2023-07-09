use crate::definitions::keywords::{Snowflake, Token::TypeAssignment, Types};

pub fn handler(tokens: &mut Vec<Snowflake>, mut pos: usize, final_file: &mut Vec<String>) -> usize {
    let name = &tokens[pos + 1].value;
    pos += match &tokens[pos + 2].value_type {
        Types::Token(TypeAssignment) => 4,
        _ => 2,
    };
    let value_token = &tokens[pos];
    match value_token.value_type {
        Types::String => final_file.push(format!(
            "let mut {name}: String = \"{}\".to_string();",
            value_token.value
        )),
        Types::I8 => final_file.push(format!("let mut {name}: i8 = {};", value_token.value)),
        Types::I16 => final_file.push(format!("let mut {name}: i16 = {};", value_token.value)),
        Types::I32 => final_file.push(format!("let mut {name}: i32 = {};", value_token.value)),
        _ => (),
    }
    pos
}
