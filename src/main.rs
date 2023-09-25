mod lexer;
mod parser;
mod token;

fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap();
    if filename.ends_with(".snw") {
        let file = std::fs::read_to_string(format!("{}", filename)).unwrap();
        let tokens = lexer::lex(file);
        if args.nth(0).unwrap_or("".to_string()) == "-t" {
            for token in &tokens {
                token.print(0);
            }
        }
        let final_file = format!(
            "#[allow(unused_variables,unused_mut,redundant_semicolons,unused_must_use)]fn main(){{{}}}",
            parser::parse(tokens)
        );
        if args.nth(0).unwrap_or("".to_string()) == "-p" {
            println!("{final_file}");
        }
        std::fs::write(
            format!("{}.rs", filename.split(".snw").nth(0).unwrap()),
            final_file,
        )
        .unwrap()
    }
}
