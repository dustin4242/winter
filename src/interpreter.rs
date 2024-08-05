use std::collections::HashMap;

use crate::lexer::Token;

pub fn interpret(tokens: Vec<Token>) {
    println!("{tokens:?}");
    let mut variables: Vec<HashMap<String, Token>> = Vec::new();
    variables.push(HashMap::<String, Token>::new());
    let mut token_index = 0;
    loop {
        if let Some(token) = tokens.get(token_index) {
            match token {
                Token::Comment => (),
                Token::Let => {
                    if let Token::Word(variable_name) = tokens.get(token_index + 1).unwrap() {
                        token_index += 3;
                        let (variable_value, index_increase) =
                            parse_multiple_token(token_index, &tokens, variables.to_owned());
                        token_index += index_increase;
                        let scope = variables.len() - 1;
                        let scope_variables = variables.get_mut(scope).unwrap();
                        scope_variables.insert(
                            variable_name.to_owned(),
                            variable_value.get(0).unwrap().to_owned(),
                        );
                    }
                }
                _ => (),
            }
            token_index += 1;
        } else {
            break;
        }
    }
    println!("{variables:?}");
}

fn parse_multiple_token(
    current_index: usize,
    tokens: &Vec<Token>,
    variables: Vec<HashMap<String, Token>>,
) -> (Vec<Token>, usize) {
    let mut index_increase = 0;
    let full_token;
    loop {
        let token = tokens
            .get(current_index.to_owned() + 1 + index_increase * 2)
            .unwrap();
        match token {
            Token::Operator("Add")
            | Token::Operator("Multiply")
            | Token::Operator("Divide")
            | Token::Operator("Subtract") => {
                index_increase += 1;
            }
            _ => break,
        }
    }
    full_token = tokens[current_index..current_index + 1 + index_increase * 2].to_vec();
    (full_token, index_increase * 2)
}
