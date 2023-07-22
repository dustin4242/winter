use crate::definitions::keywords::{Snowflake, Types};

pub fn handler(tokens: &mut Vec<Snowflake>, pos: usize, final_file: &mut Vec<String>) -> usize {
    let current = tokens[pos].value.to_owned();
    let mut next = &mut tokens[pos + 1];
    if current == "+" && next.value_type == Types::String {
        next.value = next.value.split(".to_owned()").nth(0).unwrap().to_owned();
    }
    final_file.push(tokens[pos].value.to_owned());
    pos
}
