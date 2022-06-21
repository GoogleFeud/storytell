use std::fmt::{Display, Formatter, Result};


/// Both `col` and `line` start from 1. If the location doesn't exist, then
/// both will be equal to 0.
#[derive(PartialEq, Default, Debug, Clone, PartialOrd)]
pub struct Location {
    pub col: usize,
    pub line: usize
}

/// All Ranges should be **exclusive**.
#[derive(PartialEq, Default, Debug, Clone, PartialOrd)]
pub struct Range {
    pub start: Location,
    pub end: Location
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}:{} - {}:{})", self.start.line, self.start.col, self.end.line, self.end.col)
    }
}