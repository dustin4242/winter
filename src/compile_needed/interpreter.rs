use crate::{
    compile_needed::tokenizer,
    definitions::keywords::{Snowflake, Token::*, Types},
};
use std::{collections::HashMap, fs::read_to_string, io::Write};

#[allow(dead_code)]
struct Variable {
    var_type: Types,
    value: String,
}
impl Variable {
    fn new(var_type: Types, value: String) -> Variable {
        Variable { var_type, value }
    }
}

pub fn run() {
    let mut variables: HashMap<String, Variable> = HashMap::new();
    let mut functions: HashMap<String, String> = HashMap::new();

    let mut pos: usize;
    let mut line: String;
    let mut tokens: Vec<Snowflake>;
    loop {
        pos = 0;
        line = get_user_input();
        tokens = tokenizer::run(line.to_owned());
        while pos < tokens.len() {
            match &tokens[pos].value_type {
                Types::Token(Keyword) => match tokens[pos].value.as_str() {
                    "let" => pos = let_handler(&mut tokens, pos, &mut variables),
                    "use" => {
                        if pos + 1 > tokens.len() {
                            eprintln!("No snow file specified to use");
                            pos = tokens.len();
                            continue;
                        }
                        if tokens[pos + 1].value_type != Types::String {
                            eprintln!(
                                "{} is not a valid use descriptor",
                                tokens[pos + 1].value_type.to_string()
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
                Types::Token(Word) => {
                    let contains = (
                        variables.contains_key(&tokens[pos].value),
                        functions.contains_key(&tokens[pos].value),
                        Types::is_type(tokens[pos].value.to_owned()),
                    );
                    if !contains.0 {
                        if pos.checked_sub(1).is_some()
                            && tokens[pos - 1].value_type == Types::Token(TypeAssignment)
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
                x => unreachable!("shouldn't be here: {}", x.to_string()),
            }
            pos += 1;
        }
    }
}

fn get_user_input() -> String {
    let mut temp_string = "".to_string();
    let mut output = std::io::stdout();
    output.write_all(b"> ").unwrap();
    output.flush().unwrap();
    std::io::stdin().read_line(&mut temp_string).unwrap();
    temp_string
}

fn let_handler(
    tokens: &mut Vec<Snowflake>,
    mut pos: usize,
    variables: &mut HashMap<String, Variable>,
) -> usize {
    if pos + 4 > tokens.len() {
        if pos + 2 > tokens.len() {
            eprintln!("Missing 2 descriptors for let statement");
            pos = tokens.len() - 1;
            return pos;
        }
    }
    if tokens[pos + 1].value_type != Types::Token(Word) {
        eprintln!(
            "Expected a word but found a {}",
            tokens[pos + 1].value_type.to_string()
        );
        pos = tokens.len() - 1;
        return pos;
    }
    if tokens[pos + 2].value_type == Types::Token(TypeAssignment) {
        if tokens[pos + 3].value_type != Types::Token(Word)
            || Types::is_type(tokens[pos + 3].value.to_owned())
        {
            eprintln!("\"{}\" is not a valid type", tokens[pos + 3].value);
            pos = tokens.len() - 1;
            return pos;
        }
        let (name, variable) = (
            tokens[pos + 1].value.clone(),
            Variable::new(tokens[pos + 3].value_type, tokens[pos + 3].value.clone()),
        );
        variables.insert(name, variable);
        pos += 4;
    } else {
        let (name, variable) = (
            tokens[pos + 1].value.clone(),
            Variable::new(tokens[pos + 2].value_type, tokens[pos + 2].value.clone()),
        );
        variables.insert(name, variable);
        pos += 2;
    }
    pos
}
