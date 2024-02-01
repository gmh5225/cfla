use logos::Logos;
use crate::error::{Error, ParserResult};

pub mod error;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("<")]
    OpenAngle,
    #[token(">")]
    CloseAngle,
    #[token("=")]
    Eq,
    #[token("!")]
    Not,
    #[token("&")]
    Amp,
    #[token("|")]
    Pipe,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("?")]
    Quest,
    #[token(":")]
    Colon,
    #[token("\\")]
    Backslash,
    #[token("_")]
    Underscore,
    #[token("#")]
    Hash,
    #[token("$")]
    Dollar,
    #[token("^")]
    Caret,
    #[token("%")]
    Rem,
    #[token("@")]
    At,
    #[token(".")]
    Dot,
    #[token("~")]
    Tilde,
    #[token("`")]
    Backtick,
    Unicode,
    #[regex("\"(${\\.*}|\\.|[^\"##\\])*\"")]
    StringLiteral,
    #[regex("(0-9)+")]
    IntLiteral,
    #[regex("(0.9)+.(0-9)*")]
    FloatLiteral,
    #[regex("\'(.)\'")]
    CharLiteral,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex("u(2|4|8|16|32|64|128|size)")]
    UintType,
    #[regex("i(2|4|8|16|32|64|128|size")]
    IntType,
    #[regex("f(32|64)")]
    FloatType,
    #[token("bool")]
    BoolType,
    #[token("str")]
    StrType,
    #[token("char")]
    CharType,
}

#[derive(Debug, Clone)]
pub struct TokenStream<'a> {
    str: &'a str,
    i: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(str: &'a str) -> Self {
        Self { str, i: 0 }
    }

    pub fn expect(&mut self, str: &str) -> ParserResult<()> {
        for a in str.chars() {
            match self.next() {
                None => Err(Error::Unexpected { given: None })?,
                Some(b) if a == b => break,
                Some(b) => Err(Error::Unexpected { given: Some(b.to_string()) })?,
            }
        }

        Ok(())
    }
}

impl Iterator for TokenStream<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.str.chars().nth(self.i);
        self.i += 1;
        char
    }
}