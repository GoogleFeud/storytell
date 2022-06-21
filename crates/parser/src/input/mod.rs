
use storytell_diagnostics::location::*;

pub struct LineConsumer<'a> {
    pub data: Vec<&'a str>,
    pub line: usize,
    pub col: usize
}

impl<'a> LineConsumer<'a> {

    pub fn new(content: &'a str) -> Self {
        Self {
            line: 0,
            col: 0,
            data: content.lines().collect()
        }
    }

    pub fn consume_line(&mut self) -> Option<&'a str> {
        if self.line > self.data.len() {
            None
        } else {
            self.line += 1;
            Some(self.data[self.line])
        }
    }

    pub fn consume_char(&mut self) -> Option<char> {
        if self.col > self.data[self.line].len() {
            None
        } else {
            self.col += 1;
            
        }
    }
}