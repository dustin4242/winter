use crate::definitions::keywords::Snowflake;

pub fn handler(tokens: &mut Vec<Snowflake>, pos: usize, final_file: &mut Vec<String>) -> usize {
    final_file.push(tokens[pos + 1].value.to_owned());
    pos + 1
}
