use crate::tokenizer::{tokenizer, Snowflake};
use std::fs::read_to_string;

pub fn parser(tokens: &mut Vec<Snowflake>) -> String {
    let mut asm_file = [
        vec!["format ELF64 executable".to_string()],
        vec!["segment readable writeable".to_string()],
        vec!["segment readable executable\nentry $".to_string()],
    ];
    let mut pos = 0;
    while pos < tokens.len() {
        match tokens[pos].value_type.as_str() {
            "keyword" => match tokens[pos].value.as_str() {
                "let" => {
                    let name = &tokens[pos + 1].value;
                    if &tokens[pos + 2].value == ":" {
                        let value = &tokens[pos + 4].value;
                        let value_size = value.len();
                        asm_file[1].push(format!("{name} db {value_size},\"{value}\""));
                        pos += 4;
                    } else {
                        let value = &tokens[pos + 2].value;
                        let value_size = value.len();
                        asm_file[1].push(format!("{name} db {value_size},\"{value}\""));
                        pos += 2;
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
                    let asm = &tokens[pos + 1].value;
                    asm_file[0].push(asm.to_owned());
                    pos += 1;
                }
                _ => (),
            },
            "word" => {
                let func_name = &tokens[pos].value;
                let func_arguments = &tokens[pos + 2].value;
                asm_file[2].push(format!("{func_name}({func_arguments})"));
                pos += 2
            }
            _ => (),
        }
        pos += 1;
    }
    asm_file[2].push("mov rax, 60\nmov rdi, 0\nsyscall".to_string());
    let final_file =
        asm_file[0].join("\n") + "\n" + &asm_file[1].join("\n") + "\n" + &asm_file[2].join("\n");
    return final_file;
}
