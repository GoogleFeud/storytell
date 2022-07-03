

pub fn resolve_line_endings<'a>(len: usize) -> &'a str {
    match len {
        1 => "\n",
        _ => "\r\n"
    }
}

pub fn clone_and_empty_vector<T: Clone>(vec: &mut Vec<T>) -> Vec<T> {
    let cloned = vec.clone();
    vec.clear();
    cloned
}

pub trait ExtendedOption<T: PartialEq> {
    fn is(&self, character: T) -> bool;
}

impl<T: PartialEq> ExtendedOption<T> for Option<T> {

    fn is(&self, character: T) -> bool {
        self == &Some(character)
    }
    
}