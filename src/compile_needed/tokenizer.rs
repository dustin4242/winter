use crate::definitions::keywords::{keywords, Snowflake, Token::*, Types};

pub fn run(file: String) -> Vec<Snowflake> {
    let viable_chars =
        to_chars("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890_-".as_bytes());
    let viable_nums = to_chars("1234567890".as_bytes());
    let chars = to_chars(file.as_bytes());
    let mut tokens: Vec<Snowflake> = Vec::new();
    let mut pos = 0;
    let keywords = keywords();
    while pos < file.len() {
        match chars[pos] {
            '"' => {
                let mut token_value = "".to_string();
                while pos + 1 < file.len() && chars[pos + 1] != '"' {
                    pos += 1;
                    token_value.push(chars[pos]);
                }
                tokens.push(Snowflake::new(Types::String, token_value));
                pos += 1;
            }
            '\'' => {
                let mut token_value = "".to_string();
                while pos + 1 < file.len() && chars[pos + 1] != '\'' {
                    pos += 1;
                    token_value.push(chars[pos]);
                }
                tokens.push(Snowflake::new(Types::String, token_value));
                pos += 1;
            }
            '(' => tokens.push(Snowflake {
                value_type: Types::Token(ParenOpen),
                value: "(".to_string(),
            }),
            ')' => tokens.push(Snowflake {
                value_type: Types::Token(ParenClose),
                value: ")".to_string(),
            }),
            ':' => tokens.push(Snowflake::new(
                Types::Token(TypeAssignment),
                ":".to_string(),
            )),
            '+' => tokens.push(Snowflake::new(Types::Token(Operator), "+")),
            '-' => tokens.push(Snowflake::new(Types::Token(Operator), "-")),
            '*' => tokens.push(Snowflake::new(Types::Token(Operator), "*")),
            '/' => {
                if chars[pos + 1] != '/' {
                    tokens.push(Snowflake::new(Types::Token(Operator), "/"))
                }
                while chars[pos] != '\n' {
                    pos += 1;
                }
            }
            _ => {
                if viable_nums.contains(&chars[pos]) {
                    let mut token_value = String::new();
                    while pos < chars.len() && viable_nums.contains(&chars[pos]) {
                        token_value.push(chars[pos]);
                        pos += 1;
                    }
                    pos -= 1;
                    tokens.push(Snowflake::new(Types::I32, token_value));
                } else if viable_chars.contains(&chars[pos]) {
                    let mut token_value = "".to_string();
                    while pos < chars.len() && viable_chars.contains(&chars[pos]) {
                        token_value.push(chars[pos]);
                        pos += 1;
                    }
                    pos -= 1;
                    tokens.push(Snowflake::new(
                        if keywords.contains(&token_value) {
                            Types::Token(Keyword)
                        } else {
                            Types::Token(Word)
                        },
                        token_value,
                    ));
                }
            }
        }
        pos += 1;
    }
    tokens
}

fn to_chars(string: &[u8]) -> Vec<char> {
    string
        .into_iter()
        .map(|x| char::from_u32(*x as u32).unwrap())
        .collect()
}
