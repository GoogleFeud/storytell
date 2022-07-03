

pub fn resolve_line_endings<'a>(len: usize) -> &'a str {
    match len {
        1 => "\n",
        _ => "\r\n"
    }
}

pub trait MoveVector<T: Clone> {
    fn clone_and_empty(&mut self) -> Vec<T>;
}

impl<T: Clone> MoveVector<T> for Vec<T> {

    fn clone_and_empty(&mut self) -> Vec<T> {
        let cloned = self.clone();
        self.clear();
        cloned
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