
use std::collections::{HashMap};
use storytell_diagnostics::diagnostic::{DiagnosticCollector, Diagnostic};
use crate::files::file_host::FileHost;

/// The compiler just compiles everything to javascript
/// It doesn't provide a "runtime" which actually keeps
/// track of the current path, etc.
/// The compiler also doesn't provide any tools for analyzing
pub struct JSBootstrapVars {
    /// Name of a funtion which moves the current path,
    /// it receives an array of path names
    pub revert_fn: &'static str
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

    pub fn compile(&self, ctx: Option<CompilerContext>) -> String {
        String::new()
    }

}


pub struct Path {
    pub name: String,
    pub children: HashMap<String, Path>
}

impl Path {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: HashMap::new()
        }
    }

    pub fn create(&mut self, name: &str) {
        self.children.insert(name.to_string(), Path::new(name));
    }


}

pub struct CompilerContext<'a> {
    pub magic_variables: HashMap<String, MagicVariableType>,
    pub diagnostics: Vec<Diagnostic>,
    pub paths: Path,
    pub current_path: Option<&'a Path>,
    pub bootstrap: JSBootstrapVars
}

impl<'a> CompilerContext<'a> {

    pub fn new(bootstrap: JSBootstrapVars) -> Self {
        Self { 
            magic_variables: HashMap::new(), 
            diagnostics: vec![], 
            paths: Path::new(""),
            current_path: None,
            bootstrap 
        }
    }

}

impl<'a> DiagnosticCollector for CompilerContext<'a> {
    fn add_diagnostic(&mut self, err: Diagnostic) {
        self.diagnostics.push(err);
    }
}