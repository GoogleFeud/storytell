
use storytell_diagnostics::location::Range;

pub struct InputConsumer<'a> {
    pub data: &'a [u8],
    pub pos: usize
}

impl<'a> InputConsumer<'a> {

    pub fn new(code: &'a str) -> Self {
        Self {
            data: code.as_bytes(),
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

    pub fn skip_until_after(&mut self, character: char) {
        while self.pos < self.data.len() {
            if self.data[self.pos] == character as u8 {
                self.pos += 1;
                break;
            }
            self.pos += 1;
        }
    }

    pub fn skip_until_bool(&mut self, condition: fn(char, char) -> bool) {
        let len = self.data.len() - 1;
        while self.pos < len {
            if condition(self.data[self.pos] as char, self.data[self.pos + 1] as char) {
                self.pos += 2;
                break;
            }
            self.pos += 1;
        }
    }

    pub fn expect_next(&mut self, character: char) -> bool {
        if self.pos >= self.data.len() {
            false 
        } else {
            let item = self.data[self.pos] as char;
            self.pos += 1;
            item == character
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
        input.skip_until_after('d');
        assert_eq!(input.next(), Some('e'));
        assert_eq!(input.next(), None);
    }
}