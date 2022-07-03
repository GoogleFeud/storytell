use storytell_diagnostics::{diagnostic::Diagnostic, location::*};

use crate::ast::utils::ExtendedOption;

pub trait ParsingContext {
    fn line_endings(&self) -> usize;
    fn add_diagnostic(&mut self, diagnostic: Diagnostic);
}

pub struct InputConsumer<'a, P: ParsingContext> {
    pub data: &'a [u8],
    pub ctx: P,
    pub pos: usize
}

impl<'a, P: ParsingContext> InputConsumer<'a, P> {
    pub fn new(content: &'a str, ctx: P) -> Self {
        Self {
            pos: 0,
            ctx,
            data: content.as_bytes(),
        }
    }

    pub fn slice(&self, len: usize) -> &str {
        if (self.pos + len) > self.data.len() {

        }
        unsafe { 
            std::str::from_utf8_unchecked(&self.data[if (self.pos + len) > self.data.len() {
                self.pos..self.data.len()
            } else {
                self.pos..(self.pos + len)
            }]) 
        }
    }

    pub fn skip(&mut self) {
        self.pos += 1;
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn skip_n(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn force_next(&mut self) -> char {
        let item = self.data[self.pos] as char;
        self.pos += 1;
        item
    }

    pub fn back(&mut self, n: usize) {
        self.pos -= n;
    }

    pub fn peek(&self) -> Option<char> {
        if self.pos >= self.data.len() {
            None
        } else {
            Some(self.data[self.pos] as char)
        }
    }

    pub fn peek_n(&self, n: usize) -> Option<char> {
        let f = self.pos + n;
        if f >= self.data.len() {
            None
        } else {
            Some(self.data[f] as char)
        }
    }

    pub fn consume(&mut self) -> Option<char> {
        if self.pos >= self.data.len() {
            None
        } else {
            let item = self.data[self.pos] as char;
            self.pos += 1;
            Some(item)
        }
    }

    pub fn consume_until_end_of_line(&mut self) -> &str {
        let start = self.pos;
        while !self.is_eof() {
            match self.data[self.pos] {
                b'\n' if self.ctx.line_endings() == 1 => break,
                b'\r' if self.ctx.line_endings() == 2 && self.peek().is('\n') => break,
                _ => self.pos += 1
            }
        }
        let string = unsafe { std::str::from_utf8_unchecked(&self.data[start..self.pos]) };
        self.pos += self.ctx.line_endings();
        string
    }

    pub fn skip_until_end_of_line(&mut self) {
        while !self.is_eof() {
            match self.data[self.pos] {
                b'\n' if self.ctx.line_endings() == 1 => break,
                b'\r' if self.ctx.line_endings() == 2 && self.peek().is('\n') => break,
                _ => self.pos += 1
            }
            self.pos += 1;
        }
        self.pos += self.ctx.line_endings();
    }

    pub fn consume_until(&mut self, pattern: &str) -> Option<&str> {
        let start = self.pos;
        while !self.is_eof() {
            let mut matches = true;
            for character in pattern.chars() {
                if (self.data[self.pos] as char) != character {
                    matches = false;
                    self.pos += 1;
                    break;
                }
                self.pos += 1;
            }
            if matches {
                return Some(unsafe {
                    std::str::from_utf8_unchecked(&self.data[start..(self.pos - pattern.len())])
                });
            }
        }
        None
    }

    pub fn get_pos_of(&mut self, pattern: &str) -> Option<usize> {
        let mut pos = self.pos;
        while !self.is_eof() && (pos + pattern.len() < self.data.len()) {
            if unsafe { std::str::from_utf8_unchecked(&self.data[pos..(pattern.len() + pos)]) }
                == pattern
            {
                return Some(pos);
            } else {
                pos += 1;
            }
        }
        None
    }

    pub fn is_on_new_line(&self) -> bool {
        match self.data[self.pos - 1] {
            b'\n' if self.ctx.line_endings() == 1 => true,
            b'\r' if self.ctx.line_endings() == 2 && self.data[self.pos - 2] == b'\n' => true,
            _ => false
        }
    }

    pub fn get_identation(&self) -> (u8, usize) {
        let mut depth = 0;
        let mut pos = self.pos;
        while pos < self.data.len() {
            match self.data[pos] {
                b' ' => depth += 1,
                _ => break
            }
            pos += 1;
        }
        (depth / 4, pos)
    }

    pub fn count_while(&mut self, character: char) -> usize {
        let mut count = 0;
        while !self.is_eof() {
            if (self.data[self.pos] as char) == character {
                count += 1;
                self.pos += 1;
            } else {
                break;
            }
        }
        count
    }

    pub fn range_here(&self, start: usize) -> Range<usize> {
        Range {
            start,
            end: self.pos,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.data.len()
    }

    pub fn is_eol(&self) -> bool {
        match self.data[self.pos] {
            b'\n' if self.ctx.line_endings() == 1 => true,
            b'\r' if self.ctx.line_endings() == 2 && self.peek_n(1).is('\n') => true,
            _ => false
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
            unimplemented!("Not necessary")
        }
    }

    #[test]
    fn text_input_peek() {
        let mut input = InputConsumer::new("Hello", Context {});
        assert_eq!(input.consume(), Some('H'));
        assert_eq!(input.consume(), Some('e'));
        assert_eq!(input.pos, 2);
        assert_eq!(input.consume(), Some('l'));
        assert_eq!(input.consume(), Some('l'));
        assert_eq!(input.consume(), Some('o'));
        assert_eq!(input.consume(), None);
        assert_eq!(input.is_eof(), true);
    }

    #[test]
    fn test_consume_until() {
        let mut input = InputConsumer::new("This is a test", Context {});
        assert_eq!(input.consume_until(" "), Some("This"));
        assert_eq!(input.consume_until("a t"), Some("is "));
        assert_eq!(input.pos, 11);
        assert_eq!(input.consume(), Some('e'));
    }

    #[test]
    fn test_consume_until_end_of_line() {
        let mut input = InputConsumer::new("This is a test\nLine 2\nLine 3\nLine 4", Context {});
        assert_eq!(input.consume_until("\n"), Some("This is a test"));
        input.consume();
        assert_eq!(input.consume_until_end_of_line(), "ine 2");
        assert_eq!(input.consume_until_end_of_line(), "Line 3");
        assert_eq!(input.consume(), Some('L'));
    }
}