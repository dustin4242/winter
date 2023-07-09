use crate::definitions::keywords::{Snowflake, Types};

pub fn handler(tokens: &mut Vec<Snowflake>, pos: usize, final_file: &mut Vec<String>) -> usize {
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
