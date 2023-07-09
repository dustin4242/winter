use crate::{compile_needed::tokenizer, definitions::keywords::Snowflake};
use std::fs::read_to_string;

pub fn handler(tokens: &mut Vec<Snowflake>, mut pos: usize) -> usize {
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
