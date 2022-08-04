use storytell_diagnostics::{location::Range, diagnostic::*, make_diagnostics, dia};
use crate::input::InputConsumer;

make_diagnostics!(define [
    END_OF_STR,
    JSP1001,
    "Unexpected end of string."
]);

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    String(Range<usize>),
    Number(Range<usize>),
    Identifier(Range<usize>),
    CommaPunc, //,
    SemicolonPunc, //;
    SquareBracketOpenPunc, // [
    SquareBracketClosePunc, // ]
    PranthesisOpenPunc, // (
    ParanthesisClosePunc, // )
    PlusOp, // +
    MinusOp, // -
    StarOp, // *
    SlashOp, // /
    PercentOp, // %
    EqualsEqualsEqualsOp, // ===
    EqualsEqualsOp, // ==
    EqualsOp,
    PlusEqualsOp, // +=
    MinusEqualsOp, // -=
    StarEqualsOp, // *=
    SlashEqualsOp, // /=
    ExclamationOp, // !
    DotOp, // .
    DotDotDotOp, // ...
    LessThanOp, // <
    GreaterThanOp, // >
    LessThanEqualsOp, // <=
    GreaterThanEqualsOp, // >=
    StringLitStart,
    VoidKeyword,
    TrueKeyword,
    FalseKeyword,
    NewKeyword
}

pub struct Tokenizer<'a> {
    pub input: InputConsumer<'a>,
    pub last_token: Option<Token>,
    pub errors: Vec<Diagnostic>
}

impl<'a> Tokenizer<'a> {

    pub fn new(content: &'a str) -> Self {
        Self { 
            input: InputConsumer::new(content),
            last_token: None,
            errors: vec![]
        }
    }

    pub fn parse_string(&mut self, end_char: char) -> Option<Token> {
        let start = self.input.pos - 1;
        loop {
            match self.input.next() {
                Some(character) if character == end_char => break,
                None => {
                    self.errors.push(dia!(END_OF_STR, self.input.range(start)))
                },
                _ => {}
            }
        }
        Some(Token::String(self.input.range(start)))
    }

    pub fn consume(&mut self) -> Option<Token> {
        if self.input.is_eof() {
            return None;
        }
        match self.input.next()? {
            '`' => Some(Token::StringLitStart),
            ',' => Some(Token::CommaPunc),
            ';' => Some(Token::SemicolonPunc),
            '[' => Some(Token::SquareBracketOpenPunc),
            ']' => Some(Token::SquareBracketClosePunc),
            '(' => Some(Token::PranthesisOpenPunc),
            ')' => Some(Token::ParanthesisClosePunc),
            '+' if self.input.is_next(b'=', 1) => Some(Token::PlusEqualsOp),
            '+' => Some(Token::PlusOp),
            '-' if self.input.is_next(b'=', 1) => Some(Token::MinusEqualsOp),
            '-' => Some(Token::MinusOp),
            '*' if self.input.is_next(b'=', 1) => Some(Token::StarEqualsOp),
            '*' => Some(Token::StarOp),
            '/' if self.input.is_next(b'=', 1) => Some(Token::SlashEqualsOp),
            '/' => Some(Token::SlashOp),
            '%' => Some(Token::PercentOp),
            '=' if self.input.is_next(b'=', 1) && self.input.is_next(b'=', 2) => Some(Token::EqualsEqualsEqualsOp),
            '=' if self.input.is_next(b'=', 1) => Some(Token::EqualsEqualsOp),
            '=' => Some(Token::EqualsOp),
            '!' => Some(Token::ExclamationOp),
            '.' if self.input.is_next(b'.', 1) && self.input.is_next(b'.', 2) => Some(Token::DotDotDotOp),
            '.' => Some(Token::DotOp),
            '>' if self.input.is_next(b'=', 1) => Some(Token::GreaterThanEqualsOp),
            '>' => Some(Token::GreaterThanOp),
            '<' if self.input.is_next(b'=', 1) => Some(Token::LessThanEqualsOp),
            '<' => Some(Token::LessThanOp),
            '"' => self.parse_string('"'),
            '\'' => self.parse_string('\''),
            _ => None
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.last_token.is_some() {
            self.last_token.take()
        } else {
            self.consume()
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.last_token.is_some() {
            self.last_token.as_ref()
        } else {
            self.last_token = self.consume();
            self.last_token.as_ref()
        }
    }


}