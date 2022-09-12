use storytell_diagnostics::diagnostic::Diagnostic;

use crate::file::{Directory};

use super::file::File;
use std::fs::{read_dir, write, rename};
use std::path::{PathBuf};
use rustc_hash::FxHashMap;

pub struct FileDiagnostic {
    pub diagnostics: Vec<Diagnostic>,
    pub file_id: u16
}

pub trait FileHost {
    fn write_file(&mut self, id: u16, content: String) -> bool;
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
    pub line_endings: usize
}

impl SysFileHost {
    pub fn new(line_endings: usize) -> Self {
        Self {
            files: FxHashMap::default(),
            directories: FxHashMap::default(),
            counter: 0,
            line_endings
        }
    }

    pub fn load_directory(&mut self, main: &str) -> Vec<u16> {
        let mut files_and_dirs: Vec<u16> = vec![];
        for entry in read_dir(main).unwrap().flatten() {
            if entry.file_type().unwrap().is_dir() {
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                let current_id = self.counter;
                self.counter += 1;
                let children = self.load_directory(path_str);
                self.directories.insert(current_id, Directory { 
                    id: current_id, 
                    path: path_str.to_string(), 
                    children
                });
                files_and_dirs.push(current_id);
            } else {
                self.files.insert(self.counter, File::empty(self.counter, entry.path().to_str().unwrap()));
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
            write(&file.path, content).is_ok()
        } else {
            false
        }
    }

    fn parse_file_by_id(&mut self, id: u16) -> Option<&mut File> {
        if let Some(file) = self.files.get_mut(&id) {
            file.parse(self.line_endings);
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
            let new_path = PathBuf::from(&file.path).parent().unwrap().join(name).to_str().unwrap().to_string();
            rename(&file.path, &new_path).unwrap();
            file.path = new_path.clone();
            Some(new_path)
        } else if let Some(dir) = self.directories.get_mut(&path) {
            let new_path = PathBuf::from(&dir.path).parent().unwrap().join(name).to_str().unwrap().to_string();
            rename(&dir.path, &new_path).unwrap();
            dir.path = new_path.clone();
            // TODO: Change the path of all children...
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
