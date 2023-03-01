mod keywords;
mod parser;
mod tokenizer;
use parser::parser;
use std::{
    env,
    fs::{read_to_string, write},
    io::Error,
};
use tokenizer::tokenizer;

fn main() -> Result<(), Error> {
    let filename = env::args().nth(1).expect("Didn't Provide File");
    let file = read_to_string("./".to_string() + &filename)?;
    let tokens = tokenizer(file);
    let final_file = parser(tokens);
    write(
        filename.split(".").nth(0).unwrap().to_string() + ".fasm",
        final_file,
    )?;

    Ok(())
}
