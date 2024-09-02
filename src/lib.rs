use std::{borrow::Cow, fmt};

use miette::{Error, LabeledSpan};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token<'de> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    String(&'de str),
}

impl Token<'_> {
    pub fn unescape<'de>(s: &'de str) -> Cow<'de, str> {
        todo!()
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::LeftParen => "LEFT_PAREN ( null",
                Token::RightParen => "RIGHT_PAREN ) null",
                Token::LeftBrace => "LEFT_BRACE { null",
                Token::RightBrace => "RIGHT_BRACE } null",
                Token::Comma => "COMMA , null",
                Token::Dot => "DOT . null",
                Token::Minus => "MINUS - null",
                Token::Plus => "PLUS + null",
                Token::Semicolon => "SEMICOLON ; null",
                Token::Star => "STAR * null",
                Token::String(s) => return write!(f, "STRING \"{s}\" {}", Token::unescape(s)),
            }
        )
    }
}

pub struct Lexer<'de> {
    whole: &'de str,
    rest: &'de str,
    byte: usize,
}

impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            whole: input,
            rest: input,
            byte: 0,
        }
    }
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token<'de>, Error>;

    /// Once the iterator returns `Err`, it will only return `None`.
    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.rest.chars();
        let c = chars.next()?;
        self.rest = chars.as_str();
        self.byte += c.len_utf8();

        match c {
            '(' => return Some(Ok(Token::LeftParen)),
            ')' => return Some(Ok(Token::RightParen)),
            '{' => return Some(Ok(Token::LeftBrace)),
            '}' => return Some(Ok(Token::RightBrace)),
            ',' => return Some(Ok(Token::Comma)),
            '.' => return Some(Ok(Token::Dot)),
            '-' => return Some(Ok(Token::Minus)),
            '+' => return Some(Ok(Token::Plus)),
            ';' => return Some(Ok(Token::Semicolon)),
            '*' => return Some(Ok(Token::Star)),
            '"' => {}
            _ => {
                return Some(Err(miette::miette!(
                    labels = vec![LabeledSpan::at(
                        self.byte - c.len_utf8()..self.byte,
                        "this character"
                    ),],
                    "Unexpected token \'{c}\' in input"
                )
                .with_source_code(self.whole.to_string())))
            }
        }

        todo!()
    }
}
