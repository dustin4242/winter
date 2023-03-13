#[allow(unused)] fn main() {
fn print<T: std::fmt::Display>(msg: T) { print!("{msg}") }
let mut num: i8 = 5;
let mut hello: String = "World".to_string();
print(num);
print(hello);}
