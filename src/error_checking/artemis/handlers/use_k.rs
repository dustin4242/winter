use crate::definitions::keywords::{Snowflake, Types};
use std::collections::HashMap;

pub fn handler(tokens: &Vec<Snowflake>, pos: usize, functions: &mut HashMap<String, Vec<Types>>) {
    if pos + 1 > tokens.len() {
        panic!("No snow file specified to use");
    }
    if tokens[pos + 1].value_type != Types::String {
        panic!(
            "{} is not a valid use descriptor",
            tokens[pos + 1].value_type.to_string()
        )
    }
    functions.insert(tokens[pos + 1].value.clone(), vec![]);
}
