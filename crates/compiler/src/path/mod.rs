use std::collections::{HashMap, HashSet};
use storytell_parser::ast::model::{ASTHeader, ASTBlock};
use std::rc::Rc;
use std::cell::RefCell;

pub type PathRef = Rc<RefCell<NewPath>>;

pub struct NewPath {
    pub name: String,
    pub depth: u8,
    pub children: HashMap<String, PathRef>,
    pub labels: HashSet<String>
}

pub struct PathCollector {
    pub paths: HashMap<String, PathRef>,
    pub global_labels: HashSet<String>
}

pub struct Path {
    pub name: String,
    pub depth: u8,
    pub children: HashMap<String, Path>
}

impl Path {
    pub fn new(name: &str) -> Self {
        Path { name: name.to_string(), depth: 0, children: HashMap::new() }
    }

    pub fn add_child_ast(&mut self, ast: &ASTHeader) {
        let path_name = Self::canonicalize_name(&ast.title.text);
        let mut path = Self {
            name: path_name.clone(),
            depth: ast.depth,
            children: HashMap::new()
        };
        for child in &ast.children {
            if let ASTBlock::Header(block) = child {
                path.add_child_ast(block);
            }
        }
        self.children.insert(path_name, path);
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