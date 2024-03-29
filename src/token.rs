#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenIdentifier,
    pub value: Option<String>,
    pub children: Option<Vec<Token>>,
}
impl Token {
    pub fn new(
        token_type: TokenIdentifier,
        value: Option<String>,
        children: Option<Vec<Token>>,
    ) -> Token {
        Token {
            token_type,
            value,
            children,
        }
    }
    pub fn print(&self, scope: usize) {
        print!("({:?}", self.token_type);
        match self.value.clone() {
            Some(n) => print!(": {n:?} "),
            None => print!(" "),
        }
        match self.children.as_ref() {
            Some(children) => {
                print!("->\n");
                for child in children {
                    print!("{}", "\t".repeat(scope + 1));
                    child.print(scope + 1);
                }
                print!("{})\n", "\t".repeat(scope))
            }
            None => print!("-> None)\n"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenIdentifier {
    Let,
    Const,
    TypeAssign,
    Function,
    While,
    If,
    Elif,
    Else,
    Call,
    CloseParen,
    Variable,
    Array,
    ArrayClose,
    ArrayIndex,
    Return,
    Newline,
    Comma,
    End,
    Add,
    Subtract,
    Multiply,
    Divide,
    EqualTo,
    NotEqualTo,
    Comment,
    Rust,
    TokenType(TokenType),
}
#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    string,
    number,
}
