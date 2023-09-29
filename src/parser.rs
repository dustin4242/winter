use crate::token::{Token, TokenIdentifier as TI, TokenType as TT};

pub fn parse(tokens: &Vec<Token>) -> String {
    let mut final_file = String::new();
    for token in tokens {
        final_file.push_str(&handle_token(&token))
    }
    final_file
}

fn handle_token(token: &Token) -> String {
    match token.token_type {
        TI::Variable => handle_variable(token),
        TI::While | TI::Elif | TI::If => handle_if(token),
        TI::Else => {
            let children = token.children.as_ref().unwrap();
            let else_code = get_code(&children);
            format!("else{{{else_code}}}")
        }
        TI::TypeAssign => {
            let children = token.children.as_ref().unwrap();
            let type_string = children.get(0).unwrap().value.as_ref().unwrap();
            format!(
                ":{}",
                match type_string.as_str() {
                    "string" => "String",
                    "number" => "usize",
                    x => x,
                }
            )
        }
        TI::Array => {
            let array_contents = get_arguments(token.children.as_ref().unwrap());
            format!("vec![{}]", array_contents)
        }
        TI::ArrayIndex => {
            let array_contents = get_arguments(token.children.as_ref().unwrap());
            format!(".get({}).unwrap().to_owned()", array_contents)
        }
        TI::TokenType(TT::number) | TI::TokenType(TT::string) => {
            token.value.as_ref().unwrap().to_owned()
        }
        TI::Comma => ",".to_string(),
        TI::Newline => ";\n".to_string(),
        TI::Return => {
            let return_token = token.children.as_ref().unwrap().get(0).unwrap();
            format!("return ({}).to_owned()", expand_token(return_token))
        }
        _ => "".to_string(),
    }
}

fn handle_variable(token: &Token) -> String {
    match token.children.as_ref() {
        Some(c) => {
            let children = c;
            let child_identifier = children.get(0).unwrap();
            let string = match child_identifier.token_type {
                TI::Let => format!(
                    "let mut {}={};\n",
                    token.value.as_ref().unwrap(),
                    expand_token(children.get(1).unwrap())
                ),
                TI::Const => format!(
                    "let {}={};\n",
                    token.value.as_ref().unwrap(),
                    expand_token(children.get(1).unwrap())
                ),
                TI::Function => {
                    let function_name = token.value.as_ref().unwrap();
                    let children = token.children.as_ref().unwrap();
                    let mut function_arguments = Vec::new();
                    for x in 1..children.len() {
                        function_arguments.push(children.get(x).unwrap());
                    }
                    let function_return_type = if function_arguments
                        .get(function_arguments.len() - 1)
                        .unwrap()
                        .token_type
                        == TI::TypeAssign
                    {
                        let type_assign = function_arguments.remove(function_arguments.len() - 1);
                        let mut expanded_type = handle_token(type_assign);
                        expanded_type.replace_range(..1, "");
                        Some(format!("->{}", expanded_type))
                    } else {
                        None
                    };
                    let expanded_arguments = {
                        let mut expansion = String::new();
                        for token in function_arguments.to_owned() {
                            expansion.push_str(&expand_token(token));
                        }
                        expansion
                    };
                    let function_tokens =
                        children.get(0).as_ref().unwrap().children.as_ref().unwrap();
                    let function_code = get_code(function_tokens);
                    format!(
                        "fn {}({}){}{{{}}}",
                        function_name,
                        expanded_arguments,
                        function_return_type.unwrap_or("".to_string()),
                        function_code
                    )
                }
                TI::Call => {
                    let function_name = match token.value.as_ref().unwrap().as_str() {
                        "write" => (1, "std::fs::write(".to_owned()),
                        "push" => (
                            2,
                            format!(
                                "Vec::push(&mut {}",
                                children.get(1).as_ref().unwrap().value.as_ref().unwrap()
                            ),
                        ),
                        _ => (1, format!("{}(", token.value.as_ref().unwrap())),
                    };
                    let mut function_arguments = Vec::new();
                    for x in function_name.0..children.len() {
                        function_arguments.push(children.get(x).unwrap().to_owned());
                    }
                    let expanded_arguments = get_arguments(&function_arguments);
                    format!("{}{expanded_arguments}", function_name.1)
                }
                TI::ArrayIndex => {
                    let token_string = format!(
                        "{}[{}]={};\n",
                        token.value.as_ref().unwrap(),
                        get_arguments(child_identifier.children.as_ref().unwrap()),
                        get_arguments(&children.get(1..children.len()).unwrap().to_vec())
                    );
                    token_string
                }
                _ => format!(
                    "{}={};\n",
                    token.value.as_ref().unwrap(),
                    get_arguments(children)
                ),
            };
            string
        }
        None => token.value.as_ref().unwrap().to_owned(),
    }
}
fn handle_if(token: &Token) -> String {
    let children = token.children.as_ref().unwrap();
    let bool_expand = expand_token(children.get(0).unwrap());
    let if_code = get_code(&children.get(1..children.len()).unwrap().to_vec());
    match token.token_type {
        TI::While => format!("while {bool_expand}{{{if_code}}}"),
        TI::If => format!("if {bool_expand}{{{if_code}}}"),
        TI::Elif => format!("else if {bool_expand}{{{if_code}}}"),
        _ => unreachable!(),
    }
}
fn get_code(children: &Vec<Token>) -> String {
    let mut code = String::new();
    for token in children {
        code += &handle_token(token);
    }
    code
}
fn get_arguments(children: &Vec<Token>) -> String {
    let mut code = String::new();
    for token in children {
        code += &expand_token(token);
    }
    code
}

fn expand_token(token: &Token) -> String {
    match token.token_type {
        TI::Add | TI::Subtract | TI::Multiply | TI::Divide => {
            let children = token.children.as_ref().unwrap();
            let child_1 = children.get(0).unwrap();
            let child_2 = children.get(1).unwrap();
            format!(
                "{}.to_owned(){}&{}",
                expand_token(&child_1),
                token.value.as_ref().unwrap(),
                expand_token(&child_2)
            )
        }
        TI::EqualTo | TI::NotEqualTo => {
            let children = token.children.as_ref().unwrap();
            let child_1 = children.get(0).unwrap();
            let child_2 = children.get(1).unwrap();
            format!(
                "{}{}{}",
                expand_token(&child_1),
                match token.token_type {
                    TI::EqualTo => "==",
                    TI::NotEqualTo => "!=",
                    _ => "",
                },
                expand_token(&child_2)
            )
        }
        TI::Call => "(".to_owned(),
        TI::CloseParen => ")".to_owned(),
        TI::Variable => {
            let children = token.children.as_ref();
            match children {
                Some(c) => {
                    let code = get_arguments(c);
                    format!("{}{}", token.value.as_ref().unwrap(), code)
                }
                None => format!("{}.to_owned()", token.value.as_ref().unwrap().to_owned()),
            }
        }
        TI::Comma => ",".to_string(),
        TI::TokenType(TT::string) => token.value.as_ref().unwrap().to_owned(),
        _ => handle_token(token),
    }
}
