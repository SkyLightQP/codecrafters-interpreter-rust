use std::fmt::{Display, Formatter, Result};

pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    EqualEqual,
    Equal,
    BangEqual,
    Bang,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Slash,
    String(String),
    Number(String),
    Identifier(String),
    Reserved(String),
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Star => write!(f, "*"),
            Token::Dot => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Semicolon => write!(f, ";"),
            Token::EqualEqual => write!(f, "=="),
            Token::Equal => write!(f, "="),
            Token::BangEqual => write!(f, "!="),
            Token::Bang => write!(f, "!"),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::Slash => write!(f, "/"),
            Token::String(s) => write!(f, "{}", s.replace("\"", "")),
            Token::Number(s) => write!(f, "{:?}", s.parse::<f64>().unwrap()),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Reserved(s) => write!(f, "{}", s),
            Token::EOF => write!(f, ""),
        }
    }
}
