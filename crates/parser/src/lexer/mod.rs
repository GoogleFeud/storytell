
use storytell_diagnostics::{diagnostic::Diagnostic, location::Range};
use crate::{input::{InputConsumer}, ast::utils::ExtendedOption};

pub trait ParsingContext {
    fn line_endings(&self) -> usize;
    fn add_diagnostic(&mut self, diagnostic: Diagnostic);
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    Character(char),
    EndOfLine,
    Identation,
    EndOfFile
}

impl TokenKind {

    pub fn as_char(self) -> char {
        match self {
            Self::Character(character) => character,
            Self::EndOfLine => '\n',
            Self::Identation => ' ',
            Self::EndOfFile => ' '
        }
    }

    pub fn is(&self, equal_to: char) -> bool {
        match self {
            Self::Character(character) => *character == equal_to,
            _ => false
        }
    }
}

pub struct Lexer<'a, P: ParsingContext> {
    pub input: InputConsumer<'a>,
    pub ctx: P,
    current_token: Option<TokenKind>,
    pub is_previous_eol: bool
}

impl<'a, P: ParsingContext> Lexer<'a, P> {

    pub fn new(text: &'a str, ctx: P) -> Self {
        Self {
            ctx,
            input: InputConsumer::new(text),
            current_token: None,
            is_previous_eol: false
        }
    }

    pub fn process(&mut self) -> TokenKind {
        if let Some(character) = self.input.next() {
            match character {
                '\n' if self.ctx.line_endings() == 1 => {
                    self.is_previous_eol = true;
                    TokenKind::EndOfLine
                },
                '\r' if self.ctx.line_endings() == 2 && self.input.peek().is('\n') => {
                    self.input.skip();
                    self.is_previous_eol = true;
                    TokenKind::EndOfLine
                },
                ' ' if self.is_previous_eol => TokenKind::Identation,
                other => {
                    self.is_previous_eol = false;
                    TokenKind::Character(other)
                }
            }
        } else {
            TokenKind::EndOfFile
        }
    }

    pub fn next(&mut self) -> TokenKind {
        if self.current_token.is_some() {
            self.current_token.take().unwrap()
        } else {
            self.process()
        }
    }

    pub fn peek(&mut self) -> TokenKind {
        if self.current_token.is_some() {
            self.current_token.as_ref().unwrap().clone()
        } else {
            let next = self.process();
            self.current_token = Some(next.clone());
            next
        }
    }

    pub fn peek_raw(&mut self, n: usize) -> Option<char> {
        self.input.peek_n(n)
    }

    pub fn skip_n(&mut self, n: usize) {
        self.current_token = None;
        self.input.pos += n;
    }

    pub fn skip(&mut self) {
        self.current_token = None;
    }

    pub fn consume_until_end_of_line(&mut self) -> String {
        let mut result = String::new();
        loop {
            match self.next() {
                TokenKind::Character(character) => result.push(character),
                TokenKind::EndOfFile | TokenKind::EndOfLine => break,
                _ => {}
            }
        }
        result
    }

    pub fn skip_until_end_of_line(&mut self) {
        loop {
            match self.next() {
                TokenKind::EndOfFile | TokenKind::EndOfLine => break,
                _ => {}
            }
        }
    }

    pub fn consume_until(&mut self, pattern: &str) -> Option<&str> {
        let start = self.input.pos;
        while !self.input.is_eof() {
            let mut matches = true;
            for character in pattern.chars() {
                if self.next() != TokenKind::Character(character) {
                    matches = false;
                    break;
                }
            }
            if matches {
                return Some(unsafe {
                    std::str::from_utf8_unchecked(&self.input.data[start..(self.input.pos - pattern.len())])
                });
            }
        }
        None
    }

    pub fn skip_identation(&mut self) -> u8 {
        let mut depth = 0;
        loop {
            match self.peek() {
                TokenKind::Identation => {
                    depth += 1;
                    self.skip();
                },
                _ => break
            }
        }
        depth / 4
    }

    pub fn pos(&self) -> usize {
        self.input.pos
    }

    pub fn range_here(&self, start: usize) -> Range<usize> {
        Range {
            start,
            end: self.input.pos,
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Context {}

    impl ParsingContext for Context {
        fn line_endings(&self) -> usize {
            1
        }

        fn add_diagnostic(&mut self, _diagnostic: Diagnostic) {
            unimplemented!("Not implemented.")
        }
    }

    impl Context {
        pub fn new() -> Self {
            Self {}
        }
    }

    #[test]
    fn text_lexer_lines() {
        let mut input = Lexer::new("This is the first line...\nThis is the second line...\nThird line...", Context::new());
        assert_eq!(input.consume_until_end_of_line(), "This is the first line...");
        assert_eq!(input.consume_until_end_of_line(), "This is the second line...");
        assert_eq!(input.consume_until_end_of_line(), "Third line...");
    }

    #[test]
    fn test_lexer_identation() {
        let mut input = Lexer::new("
This is the second line...
        This is the third line with 2 levels of identation - so 8 spaces.
    4 levels    
", Context::new());
       assert_eq!(input.consume_until_end_of_line(), "");
       assert_eq!(input.skip_identation(), 0);
       assert_eq!(input.consume_until_end_of_line(), "This is the second line...");
       assert_eq!(input.skip_identation(), 2);
       assert_eq!(input.consume_until_end_of_line(), "This is the third line with 2 levels of identation - so 8 spaces.");
       assert_eq!(input.skip_identation(), 1);
       assert_eq!(input.consume_until_end_of_line(), "4 levels    ")
    }

}