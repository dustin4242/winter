pub fn lexer(file: String) -> Vec<Token> {
    let mut chars = file.chars();
    let mut current_char = chars.next();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 0;
    while current_char.is_some() {
        match current_char.unwrap() {
            '/' => {
                let mut next_char = chars.next();
                if next_char == Some('/') {
                    tokens.push(Token::Comment);
                    while next_char != Some('\n') && next_char != None {
                        next_char = chars.next();
                    }
                } else {
                    tokens.push(Token::Operator("Divide"));
                    current_char = next_char;
                    continue;
                }
                line = line + 1;
            }
            '"' => {
                let mut string = String::new();
                let mut next_char = chars.next();
                while next_char.unwrap_or('\n') != '"' && next_char.unwrap_or('\n') != '\n' {
                    string.push(next_char.unwrap());
                    next_char = chars.next();
                }
                if next_char.unwrap_or('\n') == '\n' {
                    panic!("Missing \" On Line {line}")
                }
                tokens.push(Token::WString(string));
            }
            '=' => tokens.push(Token::Assign),
            '+' => tokens.push(Token::Operator("Add")),
            '-' => tokens.push(Token::Operator("Subtract")),
            '*' => tokens.push(Token::Operator("Multiply")),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::ClosedParen),
            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::ClosedBracket),
            ',' => tokens.push(Token::Comma),
            ':' => tokens.push(Token::Colon),
            '\n' => {
                line = line + 1;
                tokens.push(Token::Newline);
            }
            '\t' | ' ' => {}
            _ => {
                if current_char.unwrap().is_alphabetic() {
                    let mut word = String::new();
                    word.push(current_char.unwrap());
                    let mut next_char = chars.next();
                    while next_char.unwrap_or('\n').is_alphanumeric() {
                        word.push(next_char.unwrap());
                        next_char = chars.next();
                    }
                    tokens.push(check_keyword(word));
                    current_char = next_char;
                    continue;
                } else if current_char.unwrap().is_numeric() {
                    let mut num = String::new();
                    num.push(current_char.unwrap());
                    let mut next_char = chars.next();
                    while next_char.unwrap_or('\n').is_numeric() {
                        num.push(next_char.unwrap());
                        next_char = chars.next();
                    }
                    if next_char.unwrap_or('\n') == '.' {
                        num.push(next_char.unwrap());
                        next_char = chars.next();
                        while next_char.unwrap_or('\n').is_numeric() {
                            num.push(next_char.unwrap());
                            next_char = chars.next();
                        }
                        tokens.push(Token::Float(num));
                    } else {
                        tokens.push(Token::Number(num));
                    }
                    current_char = next_char;
                    continue;
                } else {
                    panic!("Unknown Character: {}", current_char.unwrap());
                }
            }
        }
        current_char = chars.next();
    }
    tokens
}

fn check_keyword(word: String) -> Token {
    match word.as_str() {
        "let" => Token::Let,
        "if" => Token::If,
        "elif" => Token::Elif,
        "else" => Token::Else,
        "function" => Token::Function,
        "end" => Token::End,
        "write" => Token::Write,
        _ => Token::Word(word),
    }
}

#[derive(Debug)]
pub enum Token {
    Newline,
    Comment,
    Let,
    Assign,
    OpenParen,
    ClosedParen,
    OpenBracket,
    ClosedBracket,
    Comma,
    Colon,
    If,
    Elif,
    Else,
    Function,
    End,
    Write,
    WString(String),
    Word(String),
    Float(String),
    Number(String),
    Operator(&'static str),
}
