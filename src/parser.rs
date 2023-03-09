use crate::tokenizer::{tokenizer, Snowflake};
use std::fs::read_to_string;

pub fn parser(tokens: &mut Vec<Snowflake>) -> String {
    let mut final_file = vec!["#[allow(unused)] fn main() {".to_string()];
    let mut pos = 0;
    while pos < tokens.len() {
        match tokens[pos].value_type.as_str() {
            "keyword" => match tokens[pos].value.as_str() {
                "let" => {
                    let name = &tokens[pos + 1].value;
                    pos += if &tokens[pos + 2].value_type == "type_assignment" {
                        4
                    } else {
                        2
                    };
                    let value_token = &tokens[pos];
                    match value_token.value_type.as_str() {
                        "string" => final_file.push(format!(
                            "let mut {name}: String = \"{}\".to_string();",
                            value_token.value
                        )),
                        "i8" => {
                            final_file.push(format!("let mut {name}: i8 = {};", value_token.value))
                        }
                        _ => (),
                    }
                }
                "use" => {
                    tokens.remove(pos);
                    let append_tokens = tokenizer(
                        read_to_string(format!("./snowfiles/{}.snw", tokens[pos].value)).unwrap(),
                    );
                    tokens.remove(pos);
                    for i in 0..append_tokens.len() {
                        let value = &append_tokens[i];
                        tokens.insert(pos + i, value.clone());
                    }
                    continue;
                }
                "export" => {
                    final_file.push(tokens[pos + 1].value.to_owned());
                }
                _ => unreachable!(),
            },
            "word" => {
                let func_name = &tokens[pos].value;
                let func_arguments = &tokens[pos + 2].value;
                final_file.push(format!("{func_name}({func_arguments});"));
                pos += 2
            }
            _ => (),
        }
        pos += 1;
    }
    return final_file.join("\n") + "}\n";
}
