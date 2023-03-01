mod keywords;
mod tokenizer;
use std::{fs::read_to_string, io::Error};
use tokenizer::tokenizer;

fn main() -> Result<(), Error> {
    let file = read_to_string("./main.snw")?;
    let tokens = tokenizer(file);
    println!("{tokens:?}");
    Ok(())
}
