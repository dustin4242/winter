use crate::token::{Token, TokenIdentifier as TI, TokenType as TT};

pub fn lex(file: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars: Vec<char> = file.chars().collect();
    loop {
        let new_token = parse_token(&mut chars, &mut tokens, 0);
        match new_token {
            Some(t) => tokens.push(t),
            None => break,
        }
    }
    tokens
}
fn parse_token(chars: &mut Vec<char>, tokens: &mut Vec<Token>, scope: usize) -> Option<Token> {
    let char = if chars.len() == 0 {
        return None;
    } else {
        chars.remove(0)
    };

    match char {
        '\t' | ' ' => parse_token(chars, tokens, scope),
        ',' => Some(Token::new(TI::Comma, None, None)),
        '\n' => Some(Token::new(TI::Newline, None, None)),
        '/' => {
            let next_char = chars.remove(0);
            if next_char == '/' {
                loop {
                    if chars.get(0).unwrap() == &'\n' {
                        break Some(tokens.pop().unwrap_or(Token::new(TI::Comment, None, None)));
                    } else {
                        chars.remove(0);
                    }
                }
            } else {
                chars.insert(0, next_char);
                let previous_token = tokens.pop().unwrap();
                Some(Token::new(TI::Divide, Some(format!("{char}")), {
                    let mut new_tokens: Vec<Token> = vec![previous_token];
                    let new_token = parse_token(chars, &mut new_tokens, scope);
                    new_tokens.push(new_token.unwrap());
                    Some(new_tokens)
                }))
            }
        }
        '+' | '-' | '*' => {
            let previous_token = tokens.pop().unwrap();
            Some(Token::new(
                match char {
                    '+' => TI::Add,
                    '-' => TI::Subtract,
                    '/' => TI::Divide,
                    _ => TI::Multiply,
                },
                Some(format!("{char}")),
                {
                    let mut new_tokens: Vec<Token> = vec![previous_token];
                    let new_token = parse_token(chars, &mut new_tokens, scope);
                    new_tokens.push(new_token.unwrap());
                    Some(new_tokens)
                },
            ))
        }
        ':' => {
            let previous_token = tokens.pop();
            match previous_token {
                Some(mut t) => {
                    match t.children.as_mut() {
                        Some(x) => {
                            x.push(Token::new(
                                TI::TypeAssign,
                                None,
                                Some(vec![parse_token(chars, tokens, scope).unwrap()]),
                            ));
                        }
                        None => {
                            t.children = Some(vec![Token::new(
                                TI::TypeAssign,
                                None,
                                Some(vec![parse_token(chars, tokens, scope).unwrap()]),
                            )])
                        }
                    }
                    Some(t)
                }
                None => Some(Token::new(
                    TI::TypeAssign,
                    None,
                    Some(vec![parse_token(chars, tokens, scope).unwrap()]),
                )),
            }
        }
        '!' => {
            if *chars.get(0).unwrap() == '=' {
                chars.remove(0);
                let previous_token = tokens.pop().unwrap();
                let next_token = parse_token(chars, tokens, scope).unwrap();
                Some(Token::new(
                    TI::NotEqualTo,
                    None,
                    Some(vec![previous_token, next_token]),
                ))
            } else {
                panic!("Unfinished Not Equals")
            }
        }
        '=' => {
            if *chars.get(0).unwrap() == '=' {
                chars.remove(0);
                let previous_token = tokens.pop().unwrap();
                let next_token = parse_token(chars, tokens, scope).unwrap();
                Some(Token::new(
                    TI::EqualTo,
                    None,
                    Some(vec![previous_token, next_token]),
                ))
            } else {
                let mut new_tokens: Vec<Token> = Vec::new();
                let mut new_token = parse_token(chars, &mut new_tokens, scope).unwrap();
                while new_token.token_type != TI::Newline {
                    new_tokens.push(new_token);
                    new_token = parse_token(chars, &mut new_tokens, scope).unwrap();
                }
                let mut previous_token = tokens.pop().unwrap();
                let mut children = match &previous_token.children {
                    Some(_) => previous_token.children,
                    None => Some(Vec::new()),
                };
                for token in new_tokens {
                    children.as_mut().unwrap().push(token);
                }
                previous_token.children = children;
                Some(previous_token)
            }
        }
        '"' => {
            let mut string = "\"".to_string();
            let mut new_char = chars.remove(0);
            while new_char != '"' {
                string.push(new_char);
                new_char = chars.remove(0);
            }
            Some(Token::new(
                TI::TokenType(TT::string),
                Some(string + "\".to_string()"),
                None,
            ))
        }
        '(' => {
            let mut previous_token = tokens.pop().unwrap();
            match previous_token.token_type {
                TI::Variable => {
                    let children = &previous_token.children;
                    if children == &None {
                        let mut call_arguments = vec![Token::new(TI::Call, None, None)];
                        let mut next_token =
                            parse_token(chars, &mut call_arguments, scope).unwrap();
                        while next_token.token_type != TI::CloseParen {
                            call_arguments.push(next_token);
                            next_token = parse_token(chars, &mut call_arguments, scope).unwrap();
                        }
                        previous_token.children = Some(call_arguments);
                    } else if children.as_ref().unwrap().get(0).unwrap().token_type == TI::Function
                    {
                        let function_arguments = previous_token.children.as_mut().unwrap();
                        let mut next_token = parse_token(chars, function_arguments, scope);
                        while next_token.as_ref().unwrap().token_type != TI::CloseParen {
                            function_arguments.push(next_token.unwrap());
                            next_token = parse_token(chars, function_arguments, scope);
                        }
                        next_token = parse_token(chars, &mut Vec::new(), scope);
                        if next_token.as_ref().unwrap().token_type == TI::TypeAssign {
                            let token = next_token.unwrap();
                            function_arguments.push(token);
                        }
                        let mut function_tokens = Vec::new();
                        let mut next_token = parse_token(chars, &mut function_tokens, scope);
                        while next_token != Some(Token::new(TI::End, None, None)) {
                            function_tokens.push(next_token.unwrap());
                            next_token = parse_token(chars, &mut function_tokens, scope);
                        }
                        let function = function_arguments.get_mut(0).unwrap();
                        function.children = Some(function_tokens);
                    }
                }
                TI::While | TI::Elif | TI::If => {
                    previous_token = handle_if(chars, tokens, scope, previous_token).unwrap();
                }
                _ => (),
            }
            Some(previous_token)
        }
        ')' => Some(Token::new(TI::CloseParen, None, None)),
        c => {
            if is_number(c) {
                let mut number = String::new();
                let mut next_char = chars.remove(0);
                number.push(c);
                while is_number(next_char) {
                    number.push(next_char);
                    next_char = chars.remove(0);
                }
                chars.insert(0, next_char);
                Some(Token::new(TI::TokenType(TT::i32), Some(number), None))
            } else if is_alphanumerical(c) {
                let mut token_value = String::new();
                let mut next_char = Some(&c);
                let mut x = 0;
                while is_alphanumerical(*next_char.unwrap()) {
                    token_value.push(*next_char.unwrap());
                    next_char = chars.get(x);
                    x += 1;
                }
                chars.splice(0..x - 1, []);
                match token_value.as_str() {
                    "let" => {
                        let mut parent = parse_token(chars, tokens, scope).unwrap();
                        parent.children = Some(vec![Token::new(TI::Let, None, None)]);
                        Some(parent)
                    }
                    "const" => {
                        let mut parent = parse_token(chars, tokens, scope).unwrap();
                        parent.children = Some(vec![Token::new(TI::Const, None, None)]);
                        Some(parent)
                    }
                    "function" => {
                        let mut parent = parse_token(chars, tokens, scope).unwrap();
                        parent.children = Some(vec![Token::new(TI::Function, None, None)]);
                        Some(parent)
                    }
                    "end" => Some(Token::new(TI::End, None, None)),
                    "return" => Some(Token::new(
                        TI::Return,
                        None,
                        Some(vec![parse_token(chars, tokens, scope).unwrap()]),
                    )),
                    "while" => Some(Token::new(TI::While, None, Some(Vec::new()))),
                    "if" => Some(Token::new(TI::If, None, Some(Vec::new()))),
                    "elif" => Some(Token::new(TI::Elif, None, Some(Vec::new()))),
                    "else" => handle_if(
                        chars,
                        tokens,
                        scope,
                        Token::new(TI::Else, None, Some(Vec::new())),
                    ),
                    _ => Some(Token::new(TI::Variable, Some(token_value), None)),
                }
            } else {
                panic!("Unknown char: \"{c}\" ");
            }
        }
    }
}
fn is_number(char: char) -> bool {
    "0123456789.".contains(char)
}
fn is_alphanumerical(char: char) -> bool {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".contains(char)
}
fn handle_if(
    chars: &mut Vec<char>,
    tokens: &mut Vec<Token>,
    scope: usize,
    mut previous_token: Token,
) -> Option<Token> {
    let mut if_code = Vec::new();
    let mut next_token = parse_token(chars, &mut if_code, scope);
    while next_token.as_ref().unwrap().token_type != TI::End
        && next_token.as_ref().unwrap().token_type != TI::Elif
        && next_token.as_ref().unwrap().token_type != TI::Else
    {
        if_code.push(next_token.unwrap());
        next_token = parse_token(chars, &mut if_code, scope);
    }
    for token in if_code {
        previous_token.children.as_mut().unwrap().push(token);
    }
    match next_token.as_ref().unwrap().token_type {
        TI::Elif | TI::Else => {
            tokens.push(previous_token);
            return next_token;
        }
        _ => Some(previous_token),
    }
}
