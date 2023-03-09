#[allow(unused, dead_code)]
fn main() {
fn print(msg: String) { print!("{msg}") }
fn print_num(num: i8) { print!("{}", num) }
let mut num: i8 = 5;
print_num(num)
let mut hello: String = "World".to_string();
print(hello)}
