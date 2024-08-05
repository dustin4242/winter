use std::{env::args, fs};

mod artemis;
mod interpreter;
mod lexer;

fn main() {
    let file_path = args().nth(1);
    println!("{:?}", file_path);
    let file = read_file(file_path);
    let tokens = lexer::lexer(file);
    artemis::hunt(&tokens);
    interpreter::interpret(tokens);
}

fn read_file(file_path: Option<String>) -> String {
    if let Some(f) = file_path {
        if fs::read(&f).is_ok() {
            fs::read_to_string(f).unwrap()
        } else {
            panic!("File Provided Is Not On System");
        }
    } else {
        panic!("No File Provided")
    }
}
