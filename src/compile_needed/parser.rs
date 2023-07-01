use crate::{
    compile_needed::tokenizer,
    definitions::keywords::{Snowflake, Token::*, Types},
};
use std::fs::read_to_string;

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
        Types::Token(Keyword) => match tokens[pos].value.as_str() {
            "let" => let_handler(tokens, pos, final_file),
            "use" => use_handler(tokens, pos),
            "export" => export_handler(tokens, pos, final_file),
            _ => unreachable!(),
        },
        Types::Token(Word) => word_handler(tokens, pos, final_file),
        _ => unreachable!(
            "Snowflake: {:?}, {:?}",
            tokens[pos].value_type.to_string(),
            tokens[pos].value
        ),
    }
}

// Token Handles

fn let_handler(tokens: &mut Vec<Snowflake>, mut pos: usize, final_file: &mut Vec<String>) -> usize {
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
        Types::I16 => final_file.push(format!("let mut {name}: i8 = {};", value_token.value)),
        Types::I32 => final_file.push(format!("let mut {name}: i8 = {};", value_token.value)),
        _ => (),
    }
    pos
}

fn use_handler(tokens: &mut Vec<Snowflake>, mut pos: usize) -> usize {
    let append_tokens = tokenizer::run(
        read_to_string(format!("./snowfiles/{}.snw", tokens[pos + 1].value)).unwrap(),
    );
    pos += 2;
    for i in 0..append_tokens.len() {
        let value = append_tokens[i].clone();
        tokens.insert(pos + i, value);
    }
    pos - 1
}

fn export_handler(tokens: &mut Vec<Snowflake>, pos: usize, final_file: &mut Vec<String>) -> usize {
    final_file.push(tokens[pos + 1].value.to_owned());
    pos + 1
}

fn word_handler(tokens: &mut Vec<Snowflake>, pos: usize, final_file: &mut Vec<String>) -> usize {
    let func_name = &tokens[pos].value;
    let func_arguments = &tokens[pos + 2];
    let value = &func_arguments.value;
    if func_arguments.value_type != Types::String {
        final_file.push(format!("{func_name}({value});"));
    } else {
        final_file.push(format!("{func_name}(\"{value}\");"));
    }
    pos + 3
}
