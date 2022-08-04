
use storytell_diagnostics::location::Range;
use std::ops::Index;

pub struct InputPresenter<'a> {
    pub data: &'a [u8]
}

impl<'a> InputPresenter<'a> {

    pub fn new(content: &'a str) -> Self {
        Self { 
            data: content.as_bytes()
        }
    }

    pub fn from_range(&self, range: &Range<usize>) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(&self.data[range.start..range.end])
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

}

impl<'a> Index<usize> for InputPresenter<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub struct InputConsumer<'a> {
    pub data: InputPresenter<'a>,
    pub pos: usize
}

impl<'a> InputConsumer<'a> {

    pub fn new(code: &'a str) -> Self {
        Self {
            data: InputPresenter::new(code),
            pos: 0
        }
    }

    pub fn force_next(&mut self) -> char {
        let item = self.data[self.pos] as char;
        self.pos += 1;
        item
    }

    pub fn peek(&self) -> Option<char> {
        if self.pos >= self.data.len() {
            None
        } else {
            Some(self.data[self.pos] as char)
        }
    }

    pub fn peek_nth(&self, n: usize) -> Option<char> {
        let f = self.pos + n;
        if f >= self.data.len()  {
            None 
        } else {
            Some(self.data[f] as char)
        }
    }

    pub fn skip_chars(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn skip_until_after(&mut self, character: u8) {
        while self.pos < self.data.len() {
            if self.data[self.pos] == character {
                self.pos += 1;
                break;
            }
            self.pos += 1;
        }
    }

    pub fn skip_until_bool(&mut self, condition: fn(u8, u8) -> bool) {
        let len = self.data.len() - 1;
        while self.pos < len {
            if condition(self.data[self.pos], self.data[self.pos + 1]) {
                self.pos += 2;
                break;
            }
            self.pos += 1;
        }
    }

    pub fn expect_next(&mut self, character: u8) -> bool {
        if self.pos >= self.data.len() {
            false 
        } else {
            let item = self.data[self.pos];
            self.pos += 1;
            item == character
        }
    }

    pub fn is_next(&mut self, character: u8, step: usize) -> bool {
        if self.pos >= self.data.len() {
            false 
        } else {
            (self.data[self.pos + step]) == character 
        }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.data.len()
    }

    pub fn range(&self, start: usize) -> Range<usize> {
        Range { start, end: self.pos }
    }

    pub fn range_here(&self) -> Range<usize> {
        Range { start: self.pos, end: self.pos + 1 }
    }
}

impl<'a> Iterator for InputConsumer<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.pos >= self.data.len() {
            None 
        } else {
            let item = self.data[self.pos] as char;
            self.pos += 1;
            Some(item)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut input = InputConsumer::new("abcde");
        assert_eq!(input.next(), Some('a'));
        assert_eq!(input.next(), Some('b'));
        input.skip_until_after(b'd');
        assert_eq!(input.next(), Some('e'));
        assert_eq!(input.next(), None);
    }
}