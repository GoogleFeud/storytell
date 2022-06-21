use std::fmt::{Display, Formatter, Result};


/// Both `col` and `line` start from 1. If the location doesn't exist, then
/// both will be equal to 0.
#[derive(PartialEq, Default, Debug, Clone, PartialOrd)]
pub struct Location {
    pub col: usize,
    pub line: usize
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}:{})", self.line, self.col)
    }
}

impl Location {

    pub fn exists(&self) -> bool {
        self.line != 0 && self.col != 0
    }

}

pub trait Position {
    fn pos(&self) -> usize;
    fn loc(&self, text: &[&str], line_endings: usize) -> Location;
}

impl Position for usize {

    fn pos(&self) -> usize {
        *self
    }

    fn loc(&self, text: &[&str], line_endings: usize) -> Location {
        let mut current_ind = 0;
        for (line_ind, line) in text.iter().enumerate() {
            // The index where the line ends
            let line_end_ind = current_ind + line.len();
            // If the current position is between the start and the end of the line, then it's on this line
            if *self >= current_ind && *self <= line_end_ind {
                return Location {
                    // (end of line - position) gives us the the offset from the end of the line. Subtracting that from end of line gives us the column.
                    // + 1 because the column starts from 1.
                    col: (line.len() - (line_end_ind - self)) + 1,
                    line: line_ind + 1
                }
            } else {
                current_ind += line.len() + line_endings;
            }
        }
        Location::default()
    } 

}

/// All Ranges should be **exclusive**.
#[derive(PartialEq, Default, Debug, Clone, PartialOrd)]
pub struct Range<T: Position> {
    pub start: T,
    pub end: T
}

#[derive(PartialEq, Default, Debug, PartialOrd, Clone)]
pub struct FullRange {
    pub start: Location,
    pub end: Location
}

impl Display for FullRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}:{} - {}:{})", self.start.line, self.start.col, self.end.line, self.end.col)
    }
}

impl<T: Position> Display for Range<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} - {})", self.start.pos(), self.end.pos())
    }
}

impl<T: Position> Range<T> {

    pub fn to_full(&self, text: &[&str], line_endings: usize) -> FullRange {
        FullRange {
            start: self.start.loc(text, line_endings),
            end: self.end.loc(text, line_endings)
        }
    }

    pub fn into_usize(self) -> Range<usize> {
        Range { 
            start: self.start.pos(),
            end: self.end.pos()
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usize_position() {
        let text = "abcdefg".lines().collect::<Vec<&str>>();
        assert_eq!((4 as usize).loc(&text, 2), Location {
            line: 1,
            col: 5
        });
        assert_eq!((4454 as usize).loc(&text, 2), Location {
            line: 0,
            col: 0
        });
    }

    #[test]
    fn test_usize_position_multiline() {
        let text = "\r\nabcdefg\r\naaaaaaa\r\nbbbbbbb\r\ncccc".lines().collect::<Vec<&str>>();
        assert_eq!((6 as usize).loc(&text, 2), Location {
            line: 2,
            col: 5
        });

        assert_eq!((25 as usize).loc(&text, 2), Location {
            line: 4,
            col: 6
        });
    }
}