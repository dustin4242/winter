use crate::{hail, tokenizer};
use std::{collections::HashMap, fs::read_to_string};

pub fn run() {
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut functions: HashMap<String, String> = HashMap::new();
    let types = hail::types();
    loop {
        let mut pos = 0;
        let mut line: String = "".to_string();
        std::io::stdin().read_line(&mut line).unwrap();
        let tokens = tokenizer::run(line.to_owned());
        while pos < tokens.len() {
            match tokens[pos].value_type.as_str() {
                "keyword" => match tokens[pos].value.as_str() {
                    "let" => {
                        if pos + 4 > tokens.len() {
                            if pos + 2 > tokens.len() {
                                eprintln!("Missing 2 descriptors for let statement");
                                pos = tokens.len();
                                continue;
                            }
                        }
                        if tokens[pos + 1].value_type != "word".to_string() {
                            eprintln!("Expected a word but found a {}", tokens[pos + 1].value_type);
                            pos = tokens.len();
                            continue;
                        }
                        if tokens[pos + 2].value_type == "type_assignment".to_string() {
                            if tokens[pos + 3].value_type != "word".to_string()
                                || !types.contains(&tokens[pos + 3].value)
                            {
                                eprintln!("\"{}\" is not a valid type", tokens[pos + 3].value);
                                pos = tokens.len();
                                continue;
                            }
                            variables.insert(
                                tokens[pos + 1].value.clone(),
                                tokens[pos + 3].value.clone(),
                            );
                            pos += 4;
                        } else {
                            variables.insert(
                                tokens[pos + 1].value.clone(),
                                tokens[pos + 2].value_type.clone(),
                            );
                            pos += 2;
                        }
                    }
                    "use" => {
                        if pos + 1 > tokens.len() {
                            eprintln!("No snow file specified to use");
                            pos = tokens.len();
                            continue;
                        }
                        if tokens[pos + 1].value_type != "string".to_string() {
                            eprintln!(
                                "{} is not a valid use descriptor",
                                tokens[pos + 1].value_type
                            );
                            pos = tokens.len();
                            continue;
                        }
                        let function_file =
                            read_to_string(format!("snowfiles/{}.snw", tokens[pos + 1].value))
                                .unwrap();
                        let function_tokens = tokenizer::run(function_file);
                        let function = function_tokens[function_tokens
                            .iter()
                            .position(|x| x.value == "export")
                            .unwrap()
                            + 1]
                        .value
                        .to_owned();
                        functions.insert(tokens[pos + 1].value.clone(), function);
                        pos += 2;
                    }
                    x => unreachable!("shouldn't be here: {x}"),
                },
                "word" => {
                    let contains = (
                        variables.contains_key(&tokens[pos].value),
                        functions.contains_key(&tokens[pos].value),
                        types.contains(&tokens[pos].value),
                    );
                    if !contains.0 {
                        if pos.checked_sub(1).is_some()
                            && &tokens[pos - 1].value_type == "type_assignment"
                        {
                            eprintln!("Unknown type used: {}", tokens[pos].value);
                            pos = tokens.len();
                            continue;
                        } else {
                            eprintln!("Unknown variable used: {}", tokens[pos].value);
                            pos = tokens.len();
                            continue;
                        }
                    }
                    println!("what")
                }
                x => unreachable!("shouldn't be here: {x}"),
            }
            pos += 1;
        }
    }
}
