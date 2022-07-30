pub mod compile;

use std::collections::{HashMap};
use storytell_diagnostics::diagnostic::{DiagnosticCollector, Diagnostic};
use storytell_parser::ast::model::{ASTHeader, ASTBlock};
use crate::files::file_host::FileHost;

/// The compiler just compiles everything to javascript
/// It doesn't provide a "runtime" which actually keeps
/// track of the current path, etc.
/// The compiler itself also doesn't provide any tools for analyzing.
pub struct JSBootstrapVars {
    /// Name of a funtion which moves the current path,
    /// (path: string[]) => any
    pub divert_fn: &'static str,
    pub temp_divert_fn: &'static str,
    /// Responsible for creating paragraphs
    /// (text: string, attribues: Array<{name: string, params: string[]}>) => any
    pub paragraph_fn: &'static str,
    /// Responsible for creating code blocks
    /// (code: string, language: string, attribues: Array<{name: string, params: string[]}>) => any
    pub codeblock_fn: &'static str,
    /// Responsible for creating match blocks
    /// (matched: string, choices: Array<{text: string, children: Children[]}>, directChildren: Children[], kind?: string) => any
    pub match_fn: &'static str
}

pub enum MagicVariableType {
    String,
    Number,
    Bool,
    Array,
    Map
}

pub struct JSCompiler<T: FileHost> {
    pub cwd: String,
    pub host: T
}

impl<T: FileHost> JSCompiler<T> {

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
    pub fn new(ast: &ASTHeader) -> Self {
        let mut children: HashMap<String, Path> = HashMap::new();
        for child in &ast.children {
            if let ASTBlock::Header(block) = child {
                children.insert(block.title.text.clone(), Path::new(block));
            }
        }
        Self {
            depth: ast.depth,
            name: ast.title.text.clone(),
            children
        }
    }

    pub fn get_child_by_path(&self, path: &[String]) -> Option<&Path> {
        let mut found_path = self.children.get(&path[0])?;
        for p in path.iter().skip(1) {
            found_path = found_path.children.get(p)?;
        }
        Some(found_path)
    }

    pub fn try_get_child_by_path(&self, path: &Vec<String>) -> Result<&Path, usize> {
        let mut found_path = if let Some(path) = self.children.get(&path[0]) {
            path
        } else {
            return Err(0);
        };
        for ind in 1..path.len() {
            found_path = if let Some(path) = found_path.children.get(&path[ind]) {
                path
            } else {
                return Err(ind);
            }
        }
        Ok(found_path)
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

pub struct CompilerContext {
    pub magic_variables: HashMap<String, MagicVariableType>,
    pub diagnostics: Vec<Diagnostic>,
    pub paths: Vec<Path>,
    pub bootstrap: JSBootstrapVars
}

impl CompilerContext {

    pub fn new(bootstrap: JSBootstrapVars) -> Self {
        Self { 
            magic_variables: HashMap::new(), 
            diagnostics: vec![], 
            paths: vec![],
            bootstrap 
        }
    }

}

impl DiagnosticCollector for CompilerContext {
    fn add_diagnostic(&mut self, err: Diagnostic) {
        self.diagnostics.push(err);
    }
}