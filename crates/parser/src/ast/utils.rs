

pub fn resolve_line_endings<'a>(len: usize) -> &'a str {
    match len {
        1 => "\n",
        _ => "\r\n"
    }
}

pub trait ExtendedOption<T: PartialEq> {
    fn is(&self, character: T) -> bool;
}

impl<T: PartialEq> ExtendedOption<T> for Option<T> {

    fn is(&self, character: T) -> bool {
        self == &Some(character)
    }
    
}