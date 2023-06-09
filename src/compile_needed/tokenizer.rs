use crate::definitions::keywords::{keywords, Snowflake};

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
                tokens.push(Snowflake {
                    value_type: "string".to_string(),
                    value: token_value,
                });
                pos += 1;
            }
            '\'' => {
                let mut token_value = "".to_string();
                while pos + 1 < file.len() && chars[pos + 1] != '\'' {
                    pos += 1;
                    token_value.push(chars[pos]);
                }
                tokens.push(Snowflake {
                    value_type: "string".to_string(),
                    value: token_value,
                });
                pos += 1;
            }
            '(' => tokens.push(Snowflake {
                value_type: "paren_open".to_string(),
                value: "(".to_string(),
            }),
            ')' => tokens.push(Snowflake {
                value_type: "paren_close".to_string(),
                value: ")".to_string(),
            }),
            ':' => tokens.push(Snowflake {
                value_type: "type_assignment".to_string(),
                value: ":".to_string(),
            }),
            _ => {
                if viable_nums.contains(&chars[pos]) {
                    let mut token_value = "".to_string();
                    while pos < chars.len() && viable_nums.contains(&chars[pos]) {
                        token_value.push(chars[pos]);
                        pos += 1;
                    }
                    pos -= 1;
                    tokens.push(Snowflake {
                        value_type: "i8".to_string(),
                        value: token_value,
                    });
                } else if viable_chars.contains(&chars[pos]) {
                    let mut token_value = "".to_string();
                    while pos < chars.len() && viable_chars.contains(&chars[pos]) {
                        token_value.push(chars[pos]);
                        pos += 1;
                    }
                    pos -= 1;
                    tokens.push(Snowflake {
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

fn to_chars(string: &[u8]) -> Vec<char> {
    string
        .into_iter()
        .map(|x| char::from_u32(*x as u32).unwrap())
        .collect()
}
