use std::{env::args, fs};

mod artemis;
mod interpreter;
mod lexer;

fn main() {
    let file_path_argument = args().nth(1);
    let file = read_file(file_path_argument);
    let tokens = lexer::lexer(file);
    artemis::hunt(&tokens);
    interpreter::interpret(tokens);
}

fn read_file(file_path_argument: Option<String>) -> String {
    if let Some(file_path) = file_path_argument {
        if fs::read(&file_path).is_ok() {
            fs::read_to_string(file_path).unwrap()
        } else {
            panic!("File Provided Is Not On System");
        }
    } else {
        panic!("No File Provided")
    }
}
