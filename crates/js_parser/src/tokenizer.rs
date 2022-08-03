
use crate::input::InputConsumer;

#[derive(Clone, Debug)]
pub enum Token {
    String(String),
    Number(f64),
    Identifier(String),
    Operator(String),
    Punctuation(char),
    VoidKeyword,
    TrueKeyword,
    FalseKeyword,
    NewKeyword
}

pub struct Tokenizer<'a> {
    pub input: InputConsumer<'a>,
    pub last_token: Option<Token>
}

impl<'a> Tokenizer<'a> {

    pub fn new(content: &'a str) -> Self {
        Self { 
            input: InputConsumer::new(content),
            last_token: None
        }
    }

    pub fn consume(&mut self) -> Option<Token> {
        None
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