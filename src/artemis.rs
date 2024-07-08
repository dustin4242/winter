use std::any::Any;
use std::collections::HashMap;
use std::vec::IntoIter;

use crate::lexer::Token;
use crate::lexer::Token::*;

pub fn hunt(tokens: Vec<Token>) {
    let mut token_iter = tokens.into_iter();
    let current_token = token_iter.next();
    let mut words: HashMap<String, String> = HashMap::new();
    error_check(current_token.unwrap(), None, &mut token_iter, &mut words, 1);
}

fn error_check(
    current_token: Token,
    prev_token: Option<Token>,
    token_iter: &mut IntoIter<Token>,
    words: &mut HashMap<String, String>,
    mut current_line: u32,
) {
    match current_token {
        Word(_) => {
            let next_token = get_next(token_iter.next(), &current_token).unwrap();
            match next_token {
                Assign | Operator(_) | OpenParen | OpenBracket | Newline => {}
                _ => {
                    panic!("Unexpected Token After Word: {next_token:?} On Line {current_line}")
                }
            }
            error_check(
                next_token,
                Some(current_token),
                token_iter,
                words,
                current_line,
            );
        }
        WString(_) | Number(_) => {
            let next_token = get_next(token_iter.next(), &current_token).unwrap();
            match next_token {
                Operator(_) | ClosedParen | ClosedBracket | Newline => {}
                _ => {
                    panic!("Unexpected Token After Number: {next_token:?} On Line {current_line}")
                }
            }
            error_check(
                next_token,
                Some(current_token),
                token_iter,
                words,
                current_line,
            );
        }
        Operator(_) => {
            let next_token = get_next(token_iter.next(), &current_token).unwrap();
            let next_type = type_to_string(&next_token).to_owned();
            if next_type != "Unknown" {
            } else {
                panic!("Unexpected Token After Operator: {next_token:?} On Line {current_line}")
            };
            let prev_token_type = match prev_token.as_ref().unwrap() {
                Token::Word(w) => words.get(w).unwrap(),
                _ => type_to_string(prev_token.as_ref().unwrap()),
            };
            if prev_token_type != next_type {
                panic!(
                    "{:?} Doesn't Match Same Type As {:?} On Line {current_line}",
                    next_token,
                    prev_token.unwrap()
                )
            }
            error_check(
                next_token,
                Some(current_token),
                token_iter,
                words,
                current_line,
            );
        }
        Let => {
            let next_token = get_next(token_iter.next(), &current_token).unwrap();
            match next_token {
                Word(_) => {}
                _ => {
                    panic!("Unexpected Token After Let: {next_token:?} On Line {current_line}")
                }
            }
            error_check(
                next_token,
                Some(current_token),
                token_iter,
                words,
                current_line,
            );
        }
        Assign => {
            let next_token = get_next(token_iter.next(), &current_token).unwrap();
            let string = match prev_token.unwrap() {
                Token::Word(w) => w,
                _ => panic!("Unexpected Nothingness As Variable Name. On Line {current_line}"),
            };
            let next_type = type_to_string(&next_token).to_owned();
            if next_type != "Unknown" {
                words.insert(string, next_type);
            } else {
                panic!("Unexpected Token After Assign: {next_token:?} On Line {current_line}")
            }
            error_check(
                next_token,
                Some(current_token),
                token_iter,
                words,
                current_line,
            );
        }
        //OpenParen => {}
        //ClosedParen => {}
        //OpenBracket => {}
        //ClosedBracket => {}
        Comment => {
            current_line += 1;
            let next_token = get_next(token_iter.next(), &current_token);
            if let Some(n) = next_token {
                error_check(n, Some(current_token), token_iter, words, current_line)
            }
        }
        Newline => {
            current_line += 1;
            let next_token = get_next(token_iter.next(), &current_token);
            if let Some(n) = next_token {
                match n {
                    Let => {
                        error_check(n, Some(current_token), token_iter, words, current_line);
                    }
                    _ => {
                        panic!("Unexpected Token After Newline: {n:?} On Line {current_line}")
                    }
                }
            }
        }
        _ => {
            todo!("{:?}", current_token)
        }
    }
}

fn get_next<'a>(next_token: Option<Token>, current: &Token) -> Option<Token> {
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
        WString(_) => "string",
        _ => "Unknown",
    }
}
