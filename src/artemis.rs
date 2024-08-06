use std::any::Any;
use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};

use crate::lexer::Token;
use crate::lexer::Token::*;
use std::slice::Iter;

pub fn hunt(tokens: &Vec<Token>) {
    let mut token_iter = tokens.into_iter();
    let current_token = token_iter.next();
    let mut words: Vec<HashMap<String, Token>> = Vec::new();
    let mut functions: Vec<HashMap<String, Vec<Token>>> = Vec::new();
    words.push(HashMap::new());
    functions.push(HashMap::new());
    error_check(
        current_token.unwrap(),
        None,
        &mut token_iter,
        &mut words,
        &mut functions,
        0,
        1,
    );
}

fn error_check(
    current_token: &Token,
    prev_token: Option<&Token>,
    token_iter: &mut Iter<Token>,
    words: &mut Vec<HashMap<String, Token>>,
    functions: &mut Vec<HashMap<String, Vec<Token>>>,
    mut scope: usize,
    mut current_line: u32,
) {
    let mut current_token = current_token;
    match current_token {
        Word(w) => {
            let mut next_token = check_next(token_iter.next(), &current_token).unwrap();
            match next_token {
                OpenParen => {
                    if let Some(function) = find_function_value(functions, w.to_owned()) {
                        current_token = next_token;
                        next_token = check_next(token_iter.next(), &current_token).unwrap();
                        if next_token.type_id() == Token::ClosedParen.type_id() {
                            current_token = next_token;
                            next_token = check_next(token_iter.next(), &current_token).unwrap();
                        } else {
                            panic!("Unclosed Function Call On Line {current_line}")
                        }
                    }
                }
                Assign | Operator(_) | OpenBracket | Newline => {}
                _ => {
                    panic!("Unexpected Token After Word: {next_token:?} On Line {current_line}")
                }
            }
            error_check(
                &next_token,
                Some(current_token),
                token_iter,
                words,
                functions,
                scope,
                current_line,
            );
        }
        TString(_) | Number(_) => {
            let next_token = check_next(token_iter.next(), &current_token).unwrap();
            match next_token {
                Operator(_) | ClosedParen | ClosedBracket | Newline => {}
                _ => {
                    panic!("Unexpected Token After Number: {next_token:?} On Line {current_line}")
                }
            }
            error_check(
                &next_token,
                Some(current_token),
                token_iter,
                words,
                functions,
                scope,
                current_line,
            );
        }
        Operator(_) => {
            let next_token = check_next(token_iter.next(), &current_token).unwrap();
            let next_type = match next_token {
                Token::Word(word) => {
                    type_to_string(find_word_value(words, word.to_owned()).unwrap())
                }
                _ => type_to_string(&next_token),
            };
            if next_type != "Unknown" {
            } else {
                panic!("Unexpected Token After Operator: {next_token:?} On Line {current_line}")
            };
            let prev_token_type = if let Some(prev_token) = prev_token.as_ref() {
                match prev_token {
                    Token::Word(word) => {
                        type_to_string(find_word_value(words, word.to_owned()).unwrap())
                    }
                    _ => type_to_string(prev_token),
                }
            } else {
                panic!("Expected Token Before Operator On Line {current_line}");
            };
            if prev_token_type != next_type {
                panic!(
                    "{:?} Doesn't Match Same Type As {:?} On Line {current_line}",
                    prev_token.unwrap(),
                    next_token,
                )
            }
            error_check(
                &next_token,
                Some(current_token),
                token_iter,
                words,
                functions,
                scope,
                current_line,
            );
        }
        Let => {
            let next_token = check_next(token_iter.next(), &current_token).unwrap();
            match next_token {
                Word(_) => {}
                _ => {
                    panic!("Unexpected Token After {current_token:?}: {next_token:?} On Line {current_line}")
                }
            }
            error_check(
                &next_token,
                Some(current_token),
                token_iter,
                words,
                functions,
                scope,
                current_line,
            );
        }
        Assign => {
            let next_token = check_next(token_iter.next(), &current_token).unwrap();
            let string = match prev_token.unwrap() {
                Token::Word(w) => w,
                _ => panic!("Unexpected Nothingness As Variable Name. On Line {current_line}"),
            };
            words
                .get_mut(scope)
                .unwrap()
                .insert(string.to_owned(), next_token.to_owned());
            error_check(
                &next_token,
                Some(current_token),
                token_iter,
                words,
                functions,
                scope,
                current_line,
            );
        }
        //OpenParen => {}
        //ClosedParen => {}
        //OpenBracket => {}
        //ClosedBracket => {}
        Comment => {
            current_line += 1;
            let n = check_next(token_iter.next(), &current_token).unwrap();
            error_check(
                &n,
                Some(current_token),
                token_iter,
                words,
                functions,
                scope,
                current_line,
            )
        }
        Newline => {
            current_line += 1;
            if let Some(n) = check_next(token_iter.next(), &current_token) {
                match n {
                    OpenParen | OpenBracket | ClosedParen | ClosedBracket | Comma | Assign
                    | Colon | TString(_) | Number(_) | Operator(_) => {
                        panic!("Unexpected Token After {current_token:?}: {n:?} On Line {current_line}")
                    }
                    _ => {
                        error_check(
                            &n,
                            Some(current_token),
                            token_iter,
                            words,
                            functions,
                            scope,
                            current_line,
                        );
                    }
                }
            }
        }
        Function => {
            let mut current_token = current_token;
            let function_name;
            let mut n = check_next(token_iter.next(), &current_token).unwrap();
            match n {
                Word(name) => {
                    current_token = n;
                    function_name = name
                }
                _ => {
                    panic!("Unexpected Token After {current_token:?}: {n:?} On Line {current_line}")
                }
            }
            n = check_next(token_iter.next(), &current_token).unwrap();
            match n {
                OpenParen => (),
                _ => {
                    panic!("Unexpected Token After {current_token:?}: {n:?} On Line {current_line}")
                }
            }
            n = check_next(token_iter.next(), &current_token).unwrap();
            match n {
                ClosedParen => (),
                _ => {
                    panic!("Unexpected Token After {current_token:?}: {n:?} On Line {current_line}")
                }
            }
            functions
                .get_mut(scope)
                .unwrap()
                .insert(function_name.to_owned(), Vec::new());
            increase_scope(words, functions, &mut scope);
            error_check(
                check_next(token_iter.next(), &current_token).unwrap(),
                Some(n),
                token_iter,
                words,
                functions,
                scope,
                current_line,
            )
        }
        End => {
            let n = check_next(token_iter.next(), &current_token).unwrap();
            if scope > 0 {
                decrease_scope(words, functions, &mut scope);
            } else {
                panic!("Attempted to close non-existent statement");
            }
            error_check(
                &n,
                Some(current_token),
                token_iter,
                words,
                functions,
                scope,
                current_line,
            )
        }
        //Write => {}
        _ => {
            todo!("Artemis: {current_token:?}")
        }
    }
}

fn find_word_value(words: &mut Vec<HashMap<String, Token>>, word: String) -> Option<&Token> {
    for n in 0..words.len() {
        let length = words.len();
        if let Some(value) = words.get(length - 1 - n).unwrap().get(&word) {
            return Some(value);
        } else {
            continue;
        }
    }
    None
}

fn find_function_value(
    functions: &mut Vec<HashMap<String, Vec<Token>>>,
    function_name: String,
) -> Option<&Vec<Token>> {
    for n in 0..functions.len() {
        let length = functions.len();
        if let Some(value) = functions.get(length - 1 - n).unwrap().get(&function_name) {
            return Some(value);
        } else {
            continue;
        }
    }
    None
}

fn increase_scope(
    words: &mut Vec<HashMap<String, Token>>,
    functions: &mut Vec<HashMap<String, Vec<Token>>>,
    scope: &mut usize,
) {
    words.push(HashMap::new());
    functions.push(HashMap::new());
    scope.add_assign(1);
}

fn decrease_scope(
    words: &mut Vec<HashMap<String, Token>>,
    functions: &mut Vec<HashMap<String, Vec<Token>>>,
    scope: &mut usize,
) {
    words.pop();
    functions.pop();
    scope.sub_assign(1);
}

fn check_next<'a>(next_token: Option<&'a Token>, current: &Token) -> Option<&'a Token> {
    if let Some(token) = next_token {
        Some(token)
    } else {
        if current.type_id() != Newline.type_id() {
            panic!("Expected Token After {current:?}")
        } else {
            None
        }
    }
}

fn type_to_string(token: &Token) -> &'static str {
    match token {
        Word(_) => "word",
        Number(_) => "number",
        TString(_) => "string",
        _ => "Unknown",
    }
}
