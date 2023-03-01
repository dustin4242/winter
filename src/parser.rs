use std::fs::read_to_string;

use crate::tokenizer::{tokenizer, Token};

pub fn parser(mut tokens: Vec<Token>) -> String {
    let mut asm_file = [
        vec!["format ELF64 executable".to_string()],
        vec!["segment readable writeable".to_string()],
        vec!["segment readable executable\nentry $".to_string()],
    ];

    let mut pos = 0;
    while pos < tokens.len() {
        match tokens[pos].value_type.as_str() {
            "word" => {
                if tokens[pos + 1].value_type == "paren_open".to_string() {
                    let func_name = &tokens[pos].value;
                    let func_arguments = &tokens[pos + 2].value;
                    asm_file[2].push(format!("{func_name}({func_arguments})"));
                }
            }
            "keyword" => match tokens[pos].value.as_str() {
                "let" => {
                    let name = &tokens[pos + 1].value;
                    let value = &tokens[pos + 2].value;
                    let value_size = value.len() + 1;
                    asm_file[1].push(format!("{name} db {value_size},\"{value}\""));
                    pos += 2
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
                "exportasm" => {
                    let asm = &tokens[pos + 1].value;
                    asm_file[0].push(asm.to_owned());
                    pos += 1;
                }
                _ => (),
            },
            _ => (),
        }
        pos += 1;
    }
    asm_file[2].push(
        "mov rax, 60
mov rdi, 0
syscall"
            .to_string(),
    );
    let final_file =
        asm_file[0].join("\n") + "\n" + &asm_file[1].join("\n") + "\n" + &asm_file[2].join("\n");
    return final_file;
}
