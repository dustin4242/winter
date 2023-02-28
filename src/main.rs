mod keywords;
use keywords::keywords;
use std::{fs::read_to_string, io::Error};

#[derive(Debug)]
struct Token {
    value_type: String,
    value: String,
}

fn main() -> Result<(), Error> {
    let file = read_to_string("./main.snw")?;
    let keywords = keywords();
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
    while pos < file.len() {
        match chars[pos] {
            '"' => {
                //Make a string to put the token value into
                let mut token_value = "".to_string();
                //Iterate until the entire token is built
                while chars[pos + 1] != '"' && pos < file.len() {
                    pos += 1;
                    token_value.push(chars[pos]);
                }
                pos += 1;
                tokens.push(Token {
                    value_type: "string".to_string(),
                    value: token_value,
                });
            }
            _ => {
                if viable_chars.contains(&chars[pos]) {
                    //Make a string to put the token value into
                    let mut token_value = "".to_string();
                    //*Iterate until the entire token is built
                    while viable_chars.contains(&chars[pos]) && pos < chars.len() {
                        token_value.push(chars[pos]);
                        pos += 1;
                    }
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
    println!("{tokens:?}");
    Ok(())
}
