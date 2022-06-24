
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
            Some(self.data[self.line].as_bytes()[self.col] as char)
        }
    }

    pub fn skip_line(&mut self) {
        self.line += 1;
    }

    pub fn skip_char(&mut self) {
        self.col += 1;
    }

    pub fn skip_until(&mut self, thing: &str) {
        for ind in self.line..self.data.len() {
            if let Some(index) = self.data[ind].find(thing) {
                self.col = index;
                break;
            }
            self.line += 1;
        }
    }

    // Only goes to the end of the line
    pub fn consume_until_inline(&mut self, thing: &str) -> Option<&str> {
        if let Some(index) = self.data[self.line].find(thing) {
            let captured = &self.data[self.line][self.col..index];
            self.col = index;
            Some(captured)
        } else {
            None
        }
    }

    // Checks many lines, but skips line once it finds the thing.
    pub fn consume_until_block(&mut self, thing: &str) -> String {
        let mut result = String::new();
        for ind in self.line..self.data.len() {
            if let Some(index) = self.data[ind].find(thing) {
                result += &self.data[ind][0..index];
                self.col = 0;
                self.line = ind + 1;
            }
        }
        result
    }

    pub fn loc(&self) -> Location {
        Location { col: self.col, line: self.line }
    }

    pub fn range_here(&self, start: Location) -> Range {
        Range { start, end: self.loc() }
    }
    
    
}