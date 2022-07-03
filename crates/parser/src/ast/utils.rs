

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

pub struct VecStack<T> {
    pub data: Vec<Vec<T>>
}

impl<T> VecStack<T> {

    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn push_vec(&mut self, data: Vec<T>) {
        self.data.push(data);
    }

    pub fn pop_vec(&mut self) -> Vec<T> {
        self.data.pop().unwrap_or_else(|| vec![])
    }
}