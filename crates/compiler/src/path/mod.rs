use std::collections::{HashMap, HashSet};
use storytell_diagnostics::{dia, make_diagnostics, diagnostic::*};
use storytell_parser::ast::model::{ASTHeader, ASTBlock};

make_diagnostics!(define [
    PATH_EXISTS,
    P1001,
    "There is already a path with the name $."
]);

pub struct Path {
    pub name: String,
    pub depth: u8,
    pub children: HashMap<String, u32>,
    pub labels: HashSet<String>
}

#[derive(Default)]
pub struct PathCollector {
    pub main_paths: HashMap<String, u32>,
    pub paths: HashMap<u32, Path>,
    pub current_path: u32
}

impl PathCollector {
    pub fn new() -> Self {
        Self { 
            paths: HashMap::new(),
            main_paths: HashMap::new(),
            current_path: 0
        }
    }

    pub fn add_main_path(&mut self, ast: &ASTHeader) -> Result<(), Diagnostic> {
        let mut path = Path::new(&ast.canonical_title, ast.depth);
        for child in &ast.children {
            if let ASTBlock::Header(block) = child {
                if path.children.contains_key(&block.canonical_title) {
                    return Err(dia!(PATH_EXISTS, block.title.range.clone(), &block.title.text));
                }
                let added_path = self.add_path(block, &mut path)?;
                self.paths.insert(block.id, added_path);
                path.children.insert(block.canonical_title.to_string(), block.id);
            } else if let Some(label) = child.get_label() {
                path.labels.insert(label.to_string());
            }
        }
        self.paths.insert(ast.id, path);
        self.main_paths.insert(ast.canonical_title.clone(), ast.id);
        Ok(())
    }

    pub fn add_path(&mut self, ast: &ASTHeader, main: &mut Path) -> Result<Path, Diagnostic> {
        let mut path = Path::new(&ast.canonical_title, ast.depth);
        for child in &ast.children {
            if let ASTBlock::Header(block) = child {
                if path.children.contains_key(&block.canonical_title) {
                    return Err(dia!(PATH_EXISTS, block.title.range.clone(), &block.title.text));
                }
                let added_path = self.add_path(block, main)?;
                self.paths.insert(block.id, added_path);
                path.children.insert(block.canonical_title.to_string(), block.id);
            } else if let Some(label) = child.get_label() {
                main.labels.insert(label.to_string());
            }
        }
        Ok(path)
    }

    pub fn search_path(&self, search_query: &[String]) -> Result<&Path, usize> {
        // First check if the first element in the query is a main path
        let mut final_path = if let Some(main_id) = self.main_paths.get(&search_query[0]) {
            main_id
        // Otherwise, try a local path
        } else if let Some(id) = self.paths.get(&self.current_path).unwrap().children.get(&search_query[0]) {
            id
        } else {
            return Err(0)
        };
        println!("CURRENT PATH: {:?}, SELECTED PATH: {:?}, MAP: {:?}", self.current_path, final_path, self.paths.get(final_path).unwrap().children);
        for (ind, item) in search_query.iter().enumerate().skip(1) {
            final_path = if let Some(path) = self.paths.get(final_path).unwrap().children.get(item) {
                path
            } else {
                return Err(ind)
            }
        }
        Ok(self.paths.get(final_path).unwrap())
    }


}

impl Path {
    pub fn new(name: &str, depth: u8) -> Self {
        Self { 
            name: name.to_string(),
            depth,
            children: HashMap::new(),
            labels: HashSet::new()
        }
    }
}