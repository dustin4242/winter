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
        TI::TypeAssign => {
            let children = token.children.as_ref().unwrap();
            format!(": {}", children.get(0).unwrap().value.as_ref().unwrap())
        }
        TI::TokenType(TT::I32) | TI::TokenType(TT::String) => {
            token.value.as_ref().unwrap().to_owned()
        }
        TI::Add | TI::Subtract | TI::Multiply | TI::Divide => expand_token(token),
        TI::Comma => ",".to_string(),
        TI::Newline => ";".to_string(),
        TI::Return => {
            let return_token = token.children.as_ref().unwrap().get(0).unwrap();
            format!("return {}", return_token.value.as_ref().unwrap())
        }
        _ => "".to_string(),
    }
}

fn handle_variable(token: &Token) -> String {
    let children = token.children.as_ref().unwrap();
    let child_identifier = children.get(0).unwrap();
    let string = match child_identifier.token_type {
        TI::Let => format!(
            "let mut {} = {};",
            token.value.as_ref().unwrap(),
            expand_token(children.get(1).unwrap())
        ),
        TI::Const => format!(
            "let {} = {};",
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
                let type_assign_children = function_arguments
                    .remove(function_arguments.len() - 1)
                    .children
                    .as_ref()
                    .unwrap();
                Some(format!(
                    "->{}",
                    type_assign_children.get(0).unwrap().value.as_ref().unwrap()
                ))
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
            let function_tokens = children.get(0).as_ref().unwrap().children.as_ref().unwrap();
            let mut function_code = String::new();
            for token in function_tokens {
                function_code += handle_token(token).as_str();
            }
            format!(
                "fn {}({}){} {{{}}}",
                function_name,
                expanded_arguments,
                function_return_type.unwrap_or("".to_string()),
                function_code
            )
        }
        TI::Call => {
            let function_name = match token.value.as_ref().unwrap().as_str() {
                "write" => "std::fs::write",
                _ => token.value.as_ref().unwrap(),
            };
            let mut function_arguments = Vec::new();
            for x in 1..children.len() {
                function_arguments.push(children.get(x).unwrap());
            }
            let expanded_arguments = {
                let mut expansion = String::new();
                for token in function_arguments.to_owned() {
                    expansion.push_str(&handle_token(token));
                }
                expansion
            };
            format!("{}({})", function_name, expanded_arguments)
        }
        _ => panic!(
            "Variable Type Not Defined In Parser: {:?}",
            token.token_type
        ),
    };
    string
}

fn expand_token(token: &Token) -> String {
    match token.token_type {
        TI::Add | TI::Subtract | TI::Multiply | TI::Divide => {
            let children = token.children.as_ref().unwrap();
            let child_1 = children.get(0).unwrap();
            let child_2 = children.get(1).unwrap();
            format!(
                "{} {} &{}",
                expand_token(&child_1),
                token.value.as_ref().unwrap(),
                expand_token(&child_2)
            )
        }
        TI::Variable => {
            let children = token.children.as_ref();
            match children {
                Some(c) => {
                    let child = c.get(0).unwrap();
                    format!("{}{}", token.value.as_ref().unwrap(), handle_token(child))
                }
                None => token.value.as_ref().unwrap().to_owned(),
            }
        }
        TI::Comma => ",".to_string(),
        TI::TokenType(TT::String) => token.value.as_ref().unwrap().to_owned(),
        _ => handle_token(token),
    }
}
