
use std::collections::HashMap;
use storytell_diagnostics::diagnostic::{DiagnosticCollector, Diagnostic};

use crate::files::{file_host::FileHost};

pub trait CompilerContext: DiagnosticCollector {
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

pub struct Compiler<T: FileHost> {
    pub cwd: String,
    pub host: T
}

impl<T: FileHost> Compiler<T> {

    pub fn new(cwd: &str, host: T) -> Self {
        Self {
            cwd: cwd.to_string(),
            host
        }
    }

}

pub struct CompilerCtx {
    pub magic_variables: HashMap<String, MagicVariableType>,
    pub diagnostics: Vec<Diagnostic>
}

impl CompilerContext for CompilerCtx {
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

impl DiagnosticCollector for CompilerCtx {
    fn add_diagnostic(&mut self, err: Diagnostic) {
        self.diagnostics.push(err);
    }
}