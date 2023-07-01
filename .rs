#[allow(unused)] fn main() {
fn print<T: std::fmt::Display>(msg: T) { print!("{msg}") }
let mut hello: String = "world!".to_string();
print("Hello, ");
print(hello);}
