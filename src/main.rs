mod keywords;
mod parser;
mod tokenizer;
mod wolf_hunt;
use parser::parser;
use std::{
    env,
    fs::{read_to_string, write},
    io::Error,
};
use tokenizer::tokenizer;
use wolf_hunt::wolf_hunt;

fn main() -> Result<(), Error> {
    let filename = env::args().nth(1).expect("Didn't Provide File");
    let file = read_to_string("./".to_string() + &filename)?;
    let mut tokens = tokenizer(file);
    wolf_hunt(&tokens).unwrap();
    let final_file = parser(&mut tokens);
    write(
        filename.split(".").nth(0).unwrap().to_string() + ".fasm",
        final_file,
    )?;

    Ok(())
}
