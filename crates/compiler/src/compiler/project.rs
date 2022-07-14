
use std::collections::HashMap;
use crate::files::{file_host::FileHost};

pub trait CompilerContext {
    fn set_magic_var(&mut self, name: &str, value_kind: MagicVariableType);
    fn get_magic_var(&self, name: &str) -> Option<&MagicVariableType>;
    fn get_all_magic_vars(&self) -> Vec<&String>;
}

pub enum MagicVariableType {
    String,
    Number,
    Bool,
    Array,
    Map
}

pub struct Project<T: FileHost> {
    pub cwd: String,
    pub host: T,
    pub magic_variables: HashMap<String, MagicVariableType>
}

impl<T: FileHost> Project<T> {

    pub fn new(cwd: &str, host: T) -> Self {
        Self {
            magic_variables: HashMap::new(),
            cwd: cwd.to_string(),
            host
        }
    }

}

impl<T: FileHost> CompilerContext for Project<T> {
    fn set_magic_var(&mut self, name: &str, value_kind: MagicVariableType) {
        self.magic_variables.insert(name.to_string(), value_kind);
    }

    fn get_all_magic_vars(&self) -> Vec<&String> {
        self.magic_variables.keys().collect()
    }

    fn get_magic_var(&self, name: &str) -> Option<&MagicVariableType> {
        self.magic_variables.get(name)
    }
}