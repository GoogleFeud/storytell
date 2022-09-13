use storytell_diagnostics::diagnostic::Diagnostic;

use crate::file::{Directory};

use super::file::File;
use std::fs::{read_dir, write, rename, read_to_string};
use std::path::{PathBuf};
use rustc_hash::FxHashMap;

pub struct FileDiagnostic {
    pub diagnostics: Vec<Diagnostic>,
    pub file_id: u16
}

pub trait FileHost {
    fn create_file(&mut self, parent_id: u16, name: String);
    fn write_file(&mut self, id: u16, content: String) -> bool;
    fn get_path(&self, path: &[u16], name: &str) -> String;
    fn rename_file(&mut self, id: u16, name: String) -> Option<String>;
    fn parse_file_by_id(&mut self, id: u16) -> Option<&mut File>;
    fn get_all_files(&mut self) -> Vec<&mut File>;
    fn get_files_from_directory(&self, path: &str) -> Vec<String>;
    fn get_line_endings(&self) -> usize;
}

#[derive(Default)]
pub struct SysFileHost {
    pub files: FxHashMap<u16, File>,
    pub directories: FxHashMap<u16, Directory>,
    pub counter: u16,
    pub cwd: String,
    pub line_endings: usize
}

impl SysFileHost {
    pub fn new(line_endings: usize, cwd: String) -> Self {
        Self {
            files: FxHashMap::default(),
            directories: FxHashMap::default(),
            cwd,
            counter: 0,
            line_endings
        }
    }

    pub fn load_from_cwd(&mut self) -> Vec<u16> {
        self.load_directory(&self.cwd.clone(), vec![])
    }

    pub fn load_directory(&mut self, main: &str, parents: Vec<u16>) -> Vec<u16> {
        let mut files_and_dirs: Vec<u16> = vec![];
        for entry in read_dir(main).unwrap().flatten() {
            if entry.file_type().unwrap().is_dir() {
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                let current_id = self.counter;
                self.counter += 1;
                let children = self.load_directory(path_str, parents.iter().map(|i| *i).chain(std::iter::once(current_id)).collect::<Vec<u16>>());
                self.directories.insert(current_id, Directory { 
                    id: current_id,
                    name: entry.file_name().to_str().unwrap().to_string(),
                    path: parents,
                    children
                });
                files_and_dirs.push(current_id);
            } else {
                self.files.insert(self.counter, File::empty(self.counter, parents, entry.file_name().to_str().unwrap().to_string()));
                files_and_dirs.push(self.counter);
                self.counter += 1;
            }
        }
        files_and_dirs
    }

}

impl FileHost for SysFileHost {
    fn write_file(&mut self, path: u16, content: String) -> bool {
        if let Some(file) = self.files.get(&path) {
            write(&self.get_path(&file.path, &file.name), content).is_ok()
        } else {
            false
        }
    }

    fn create_file(&mut self, parent_id: u16, name: String) {
        
    }

    fn get_path(&self, path: &[u16], name: &str) -> String {
        let mut path_buf = PathBuf::from(&self.cwd);
        for path_part in path {
            path_buf.push(&self.directories.get(path_part).unwrap().name)
        }
        path_buf.push(name);
        path_buf.to_str().unwrap().to_string()
    }

    fn parse_file_by_id(&mut self, id: u16) -> Option<&mut File> {
        if let Some(file) = self.files.get_mut(&id) {
            let line_endings = self.line_endings;
            file.parse(&read_to_string(self.get_path(&file.path, &file.name)).unwrap(), line_endings);
            Some(file)
        } else {
            None
        }
    }

    fn get_all_files(&mut self) -> Vec<&mut File> {
        self.files.values_mut().collect::<Vec<&mut File>>()
    }

    fn rename_file(&mut self, path: u16, name: String) -> Option<String> {
        if let Some(file) = self.files.get_mut(&path) {
            let old_path = self.get_path(&file.path, &file.name);
            file.name = name;
            let new_path = self.get_path(&file.path, &file.name);
            rename(&old_path, new_path).unwrap();
            Some(new_path)
        } else if let Some(dir) = self.directories.get_mut(&path) {
            let old_path = self.get_path(&dir.path, &dir.name);
            dir.name = name;
            let new_path = self.get_path(&dir.path, &dir.name);
            rename(&old_path, &new_path).unwrap();
            Some(new_path)
        } else {
            None
        }
    }

    fn get_files_from_directory(&self, directory: &str) -> Vec<String> {
        let mut files: Vec<String> = vec![];
        for entry in read_dir(directory).unwrap().flatten() {
            if entry.file_type().unwrap().is_dir() {
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                files.append(&mut self.get_files_from_directory(path_str));
            } else {
                files.push(entry.path().to_str().unwrap().to_string());
            }
        }
        files
    }

    fn get_line_endings(&self) -> usize {
        self.line_endings
    }

}
