use std::fmt::{Display, Formatter, Result};
use std::cmp::PartialOrd;

/// Both `col` and `line` start from 1. If the location doesn't exist, then
/// both will be equal to 0.
#[derive(PartialEq, Default, Debug, Clone, PartialOrd, Eq)]
pub struct Location {
    pub col: usize,
    pub line: usize
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
#[derive(PartialEq, Default, Debug, Clone, PartialOrd, Eq)]
pub struct Range<T: Position> {
    pub start: T,
    pub end: T
}

impl<T: Position> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Range {
            start,
            end
        }
    }
}

impl<T: Position> Display for Range<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} - {})", self.start.pos(), self.end.pos())
    }
}