//! A big old enum that contains every variant of allowable tokens in the grammar.

use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenType {
    // 1char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // 1 / 2 char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    GreaterEqual,
    Greater,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords / logic ops etc
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    None,
    Or,
    Return,
    Super,
    Self_,
    True,
    Var,
    While,
    Call,

    // Reserved
    Print,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
