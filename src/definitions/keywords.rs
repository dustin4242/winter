#[derive(Clone)]
pub struct Snowflake {
    pub value_type: Types,
    pub value: String,
}
impl Snowflake {
    pub fn new<T: ToString>(value_type: Types, value: T) -> Snowflake {
        Snowflake {
            value_type,
            value: value.to_string(),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Types {
    String,
    I8,
    I16,
    I32,
    Token(Token),
}

#[derive(PartialEq, Copy, Clone)]
pub enum Token {
    Operator,
    Comma,
    Newline,
    TypeAssignment,
    Word,
    Keyword,
    ParenOpen,
    ParenClose,
}
impl Types {
    pub fn is_type(value: String) -> bool {
        ["i8", "i16", "i32", "string"]
            .map(|x| x.to_string())
            .contains(&value)
    }
    pub fn to_string(&self) -> String {
        match self {
            Types::String => "string",
            Types::I8 => "i8",
            Types::I16 => "i16",
            Types::I32 => "i32",
            _ => unreachable!(),
        }
        .to_string()
    }
    pub fn to_type(string: String) -> Types {
        match string.as_str() {
            "string" => Types::String,
            "i8" => Types::I8,
            "i16" => Types::I16,
            "i32" => Types::I32,
            x => panic!("unknown type: {x}"),
        }
    }
}

pub fn keywords() -> Vec<String> {
    vec!["let", "const", "use", "export", "write"]
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}
