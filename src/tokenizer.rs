use crate::keywords::keywords;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub value_type: String,
    pub value: String,
}

pub fn tokenizer(file: String) -> Vec<Token> {
    let viable_chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .as_bytes()
        .into_iter()
        .map(|x| char::from_u32(*x as u32).unwrap())
        .collect();
    let chars: Vec<char> = file
        .as_bytes()
        .into_iter()
        .map(|x| char::from_u32(*x as u32).unwrap())
        .collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut pos = 0;
    let keywords = keywords();
    while pos < file.len() {
        match chars[pos] {
            '"' => {
                //Make a string to put the token value into
                let mut token_value = "".to_string();
                //Iterate until the entire token is built
                pos += 1;
                while pos < file.len() && chars[pos] != '"' {
                    token_value.push(chars[pos]);
                    pos += 1;
                }
                tokens.push(Token {
                    value_type: "string".to_string(),
                    value: token_value,
                });
            }
            '(' => tokens.push(Token {
                value_type: "paren_open".to_string(),
                value: "(".to_string(),
            }),
            ')' => tokens.push(Token {
                value_type: "paren_close".to_string(),
                value: ")".to_string(),
            }),
            _ => {
                if viable_chars.contains(&chars[pos]) {
                    //Make a string to put the token value into
                    let mut token_value = "".to_string();
                    //*Iterate until the entire token is built
                    while pos < chars.len() && viable_chars.contains(&chars[pos]) {
                        token_value.push(chars[pos]);
                        pos += 1;
                    }
                    pos -= 1;
                    tokens.push(Token {
                        value_type: if keywords.contains(&token_value) {
                            "keyword".to_string()
                        } else {
                            "word".to_string()
                        },
                        value: token_value,
                    });
                }
            }
        }
        pos += 1;
    }
    tokens
}