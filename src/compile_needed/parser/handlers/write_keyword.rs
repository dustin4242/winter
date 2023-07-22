use crate::definitions::keywords::{Snowflake, Token::Newline, Types};

pub fn handler(tokens: &mut Vec<Snowflake>, mut pos: usize, final_file: &mut Vec<String>) -> usize {
    let mut args: Vec<String> = Vec::new();
    while tokens[pos + 1].value_type != Types::Token(Newline) {
        match tokens[pos + 1].value.as_str() {
            "(" => (),
            ")" => (),
            "," => (),
            _ => args.push(tokens[pos + 1].value.to_owned()),
        }
        pos += 1;
    }
    final_file.push(format!(
        ";std::fs::write({},format!(\"{}\",{}))",
        args.remove(0),
        "{}".to_string() + "{}".to_owned().repeat(args.len() - 1).as_str(),
        args.join(",")
    ));
    pos
}
