use std::{collections::HashMap, ops::AddAssign};

use crate::lexer::Token;

pub fn interpret(tokens: Vec<Token>) {
    let mut words: Vec<HashMap<String, Token>> = Vec::new();
    words.push(HashMap::<String, Token>::new());
    let mut token_index = 0;
    while let Some(token) = tokens.get(token_index) {
        interpret_token(token, tokens.to_owned(), &mut words, &mut token_index);
        token_index += 1;
    }
    println!("Scope 0 Variables: {words:?}");
}

fn interpret_token(
    token: &Token,
    tokens: Vec<Token>,
    words: &mut Vec<HashMap<String, Token>>,
    token_index: &mut usize,
) {
    match token {
        Token::Comment => (),
        Token::Let => {
            if let Token::Word(word_name) = tokens.get(*token_index + 1).unwrap() {
                token_index.add_assign(3);
                let (unparsed_value, index_increase) =
                    parse_multiple_token(*token_index, &tokens, words.to_owned());
                token_index.add_assign(index_increase);
                let mut word_value = unparsed_value.get(0).unwrap().to_owned();
                match word_value {
                    Token::TString(_) => {
                        word_value = handle_string_operators(unparsed_value.to_owned(), words)
                    }
                    Token::Number(_) => {
                        word_value = handle_number_operators(unparsed_value.to_owned(), words)
                    }
                    _ => {}
                }
                let scope = words.len() - 1;
                let scope_variables = words.get_mut(scope).unwrap();
                scope_variables.insert(word_name.to_owned(), word_value.to_owned());
            }
        }
        Token::Newline => (),
        _ => todo!("Interpreter: {token:?}"),
    }
}

fn handle_number_operators(
    word_value: Vec<Token>,
    words: &mut Vec<HashMap<String, Token>>,
) -> Token {
    let mut temp_number = match word_value.get(0).unwrap() {
        Token::Number(a) => a,
        _ => &0,
    }
    .to_owned();
    let mut operator_index = 0;
    loop {
        if let Some(operator) = word_value.get(1 + operator_index * 2) {
            let operator_value = match word_value.get(2 + operator_index * 2).unwrap() {
                Token::Number(value) => value,
                Token::Word(name) => match find_variable_value(words, name.to_owned()).unwrap() {
                    Token::Number(n) => n,
                    _ => &0,
                },
                _ => &0,
            };
            match operator {
                Token::Operator("Add") => temp_number += operator_value,
                Token::Operator("Subtract") => temp_number -= operator_value,
                Token::Operator("Multiply") => temp_number *= operator_value,
                Token::Operator("Divide") => temp_number /= operator_value,
                _ => {}
            }
            operator_index += 1;
        } else {
            break;
        }
    }
    Token::Number(temp_number)
}

fn handle_string_operators(
    word_value: Vec<Token>,
    words: &mut Vec<HashMap<String, Token>>,
) -> Token {
    let mut temp_string = match word_value.get(0).unwrap() {
        Token::TString(a) => a,
        _ => "",
    }
    .to_owned();
    let mut operator_index = 0;
    loop {
        if let Some(operator) = word_value.get(1 + operator_index * 2) {
            let operator_value = match word_value.get(2 + operator_index * 2).unwrap() {
                Token::TString(value) => value,
                Token::Word(name) => match find_variable_value(words, name.to_owned()).unwrap() {
                    Token::TString(n) => n,
                    _ => "",
                },
                _ => "",
            };
            match operator {
                Token::Operator("Add") => temp_string += operator_value,
                _ => {}
            }
            operator_index += 1;
        } else {
            break;
        }
    }
    Token::TString(temp_string)
}

fn find_variable_value(
    words: &mut Vec<HashMap<String, Token>>,
    word_name: String,
) -> Option<&Token> {
    for n in 0..words.len() {
        let length = words.len();
        if let Some(value) = words.get(length - 1 - n).unwrap().get(&word_name) {
            return Some(value);
        } else {
            continue;
        }
    }
    None
}

fn parse_multiple_token(
    current_index: usize,
    tokens: &Vec<Token>,
    words: Vec<HashMap<String, Token>>,
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