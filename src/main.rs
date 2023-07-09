mod compile_needed;
mod definitions;
mod error_checking;
use std::{
    env,
    fs::{read_to_string, write},
    io::Error,
};

fn main() -> Result<(), Error> {
    let filename = env::args().nth(1).expect("file not provided");
    if !filename.ends_with(".snw") {
        panic!("file \"{filename}\" is not a valid snw file")
    };
    let file = read_to_string("./".to_string() + &filename)?;
    let mut tokens = compile_needed::tokenizer::run(file);
    error_checking::artemis::hunt(&tokens);
    let final_file = compile_needed::parser::run(&mut tokens);
    write(
        filename.split(".").nth(0).unwrap().to_string() + ".rs",
        final_file,
    )?;
    Ok(())
}
