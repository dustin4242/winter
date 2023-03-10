//! # Maximum Documentation :]
//! Welcome to the Winter Documentation Page! *(or W.D.P for short)*
//!
//! This page will satisfy all your compiler tinkering needs! *(hopefully)*
mod artemis;
mod hail;
mod interpreter;
mod keywords;
mod parser;
mod tokenizer;
use std::{
    env,
    fs::{read_to_string, write},
    io::Error,
};

fn main() -> Result<(), Error> {
    let args = env::args();
    if args.len() > 1 {
        let filename = env::args().nth(1).expect("Didn't Provide File");
        let file = read_to_string("./".to_string() + &filename)?;
        let mut tokens = tokenizer::run(file);
        artemis::hunt(&tokens);
        let final_file = parser::run(&mut tokens);
        write(
            filename.split(".").nth(0).unwrap().to_string() + ".rs",
            final_file,
        )?;
    } else {
        interpreter::run();
    }
    Ok(())
}
