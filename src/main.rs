mod lexer;
mod parser;
mod token;

fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap();
    if filename.ends_with(".snw") {
        let file = std::fs::read_to_string(format!("{}", filename)).unwrap();
        let tokens = lexer::lex(file);
        let final_file = format!(
            "#[allow(unused_variables,unused_mut,redundant_semicolons,unused_must_use,non_snake_case)]fn main(){{{}}}",
            parser::parse(&tokens)
        );
        for _ in 0..args.len() {
            match args.nth(0).unwrap_or("".to_string()).as_str() {
                "-t" => {
                    for token in &tokens {
                        token.print(0);
                    }
                }
                "-p" => println!("{final_file}"),
                _ => (),
            }
        }
        std::fs::write(
            format!("{}.rs", filename.split(".snw").nth(0).unwrap()),
            final_file,
        )
        .unwrap()
    }
}
