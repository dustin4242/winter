use crate::keywords::keywords;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Snowflake {
    pub value_type: String,
    pub value: String,
}

pub fn tokenizer(file: String) -> Vec<Snowflake> {
    let viable_chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890"
        .as_bytes()
        .into_iter()
        .map(|x| char::from_u32(*x as u32).unwrap())
        .collect();
    let viable_nums: Vec<char> = "1234567890"
        .as_bytes()
        .into_iter()
        .map(|x| char::from_u32(*x as u32).unwrap())
        .collect();
    let chars: Vec<char> = file
        .as_bytes()
        .into_iter()
        .map(|x| char::from_u32(*x as u32).unwrap())
        .collect();
    let mut tokens: Vec<Snowflake> = Vec::new();
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
                tokens.push(Snowflake {
                    value_type: "string".to_string(),
                    value: token_value,
                });
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
