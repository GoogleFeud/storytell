use storytell_diagnostics::{location::*};

pub struct InputConsumer<'a> {
    pub data: &'a [u8],
    pub pos: usize,
}

impl<'a> InputConsumer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            pos: 0,
            data: content.as_bytes(),
        }
    }

    pub fn slice(&self, len: usize) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(
                &self.data[if (self.pos + len) > self.data.len() {
                    self.pos..self.data.len()
                } else {
                    self.pos..(self.pos + len)
                }],
            )
        }
    }

    pub fn skip(&mut self) {
        self.pos += 1;
    }

    pub fn skip_n(&mut self, n: usize) {
        self.pos += n;
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

    pub fn peek_n(&self, n: usize) -> Option<char> {
        let f = self.pos + n;
        if f >= self.data.len() {
            None
        } else {
            Some(self.data[f] as char)
        }
    }

    pub fn skip_chars(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn next(&mut self) -> Option<char> {
        if self.pos >= self.data.len() {
            None
        } else {
            let item = self.data[self.pos] as char;
            self.pos += 1;
            Some(item)
        }
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_input_peek() {
        let mut input = InputConsumer::new("Hello");
        assert_eq!(input.next(), Some('H'));
        assert_eq!(input.next(), Some('e'));
        assert_eq!(input.pos, 2);
        assert_eq!(input.next(), Some('l'));
        assert_eq!(input.next(), Some('l'));
        assert_eq!(input.next(), Some('o'));
        assert_eq!(input.next(), None);
        assert_eq!(input.is_eof(), true);
    }

    #[test]
    fn test_consume_until() {
        let mut input = InputConsumer::new("This is a test");
        assert_eq!(input.consume_until(" "), Some("This"));
        assert_eq!(input.consume_until("a t"), Some("is "));
        assert_eq!(input.pos, 11);
        assert_eq!(input.next(), Some('e'));
    }

}
