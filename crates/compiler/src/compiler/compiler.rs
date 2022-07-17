
use std::collections::{HashMap};
use storytell_diagnostics::diagnostic::{DiagnosticCollector, Diagnostic};
use crate::files::file_host::FileHost;

/// The compiler just compiles everything to javascript
/// It doesn't provide a "runtime" which actually keeps
/// track of the current path, etc.
/// The compiler itself also doesn't provide any tools for analyzing.
pub struct JSBootstrapVars {
    /// Name of a funtion which moves the current path,
    /// it receives an array of path names
    pub divert_fn: &'static str,
    pub temp_divert_fn: &'static str
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
    pub depth: u8,
    pub children: HashMap<String, Path>
}

impl Path {
    pub fn new(name: &str, depth: u8) -> Self {
        Self {
            depth,
            name: name.to_string(),
            children: HashMap::new()
        }
    }

    pub fn create(&mut self, name: &str) {
        self.children.insert(name.to_string(), Path::new(name, self.depth + 1));
    }

    pub fn get_child_by_path(&self, path: &Vec<String>) -> Option<&Path> {
        let mut found_path = self.children.get(&path[0])?;
        for ind in 1..path.len() {
            found_path = found_path.children.get(&path[ind])?;
        }
        Some(found_path)
    }

    /// Path names can only contain lowercase letters, digits and underscores.
    /// Empty spaces are replaced with underscores.
    /// Capital letters are replaced with their lowercase variants.
    /// Any other character gets erased.
    pub fn canonicalize_name(name: &str) -> String {
        let mut canonical = String::new();
        for character in name.chars() {
            match character {
                ' ' => canonical.push('_'),
                '_' | 'a'..='z' | '0'..='9' => canonical.push(character),
                'A'..='Z' => canonical.push(character.to_lowercase().next().unwrap()),
                _ => {}
            }
        }
        canonical
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
            paths: Path::new("", 0),
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