use storytell_diagnostics::diagnostic::Diagnostic;

use super::file::File;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, write, rename};
use std::path::{Path};

pub struct FileDiagnostic {
    pub diagnostics: Vec<Diagnostic>,
    pub filename: String
}

pub enum Blob {
    Directory(String, String, Vec<Blob>),
    File(String, String)
}

pub enum GetFindResult<'a> {
    NotFound,
    FromCache(&'a File),
    Parsed(&'a File, Option<FileDiagnostic>)
}

pub trait FileHost {
    fn write_file(&mut self, path: &str, content: String) -> bool;
    fn rename_file(&mut self, path: &str, name: String) -> Option<String>;
    fn get_or_find(&mut self, path: &str) -> GetFindResult;
    fn get_files_from_directory(&self, path: &str) -> Vec<String>;
    fn get_line_endings(&self) -> usize;
}

#[derive(Default)]
pub struct SysFileHost {
    pub files: HashMap<String, File>,
    pub line_endings: usize
}

impl SysFileHost {
    pub fn new(line_endings: usize) -> Self {
        Self {
            files: HashMap::new(),
            line_endings
        }
    }

    pub fn get_files_from_directory_as_blobs(&self, directory: &str) -> Vec<Blob> {
        let mut blobs: Vec<Blob> = vec![];
        for entry in read_dir(directory).unwrap().flatten() {
            if entry.file_type().unwrap().is_dir() {
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                blobs.push(Blob::Directory(entry.path().to_str().unwrap().to_string().replace('\\', "\\\\"), entry.file_name().to_str().unwrap().split('.').next().unwrap().to_string(), self.get_files_from_directory_as_blobs(path_str)));
            } else {
                blobs.push(Blob::File(entry.path().to_str().unwrap().to_string().replace('\\', "\\\\"), entry.file_name().to_str().unwrap().split('.').next().unwrap().to_string()));
            }
        }
        blobs
    }
}

impl FileHost for SysFileHost {
    fn write_file(&mut self, path: &str, content: String) -> bool {
        write(path, content).is_ok()
    }

    fn rename_file(&mut self, path: &str, name: String) -> Option<String> {
        let path_obj = Path::new(path);
        let new_path = path_obj.parent().unwrap().join(name + ".md").to_str().unwrap().to_string();
        // Always rename the file, even if it's not in the cache
        rename(path, &new_path).unwrap();
        if path_obj.is_dir() {
            // Remove the files from the cache - they will have to be recompiled
            for file in self.get_files_from_directory(path) {
                self.files.remove(&file);
            }
        } else {
            if let Some(file) = self.files.remove(path) {
                self.files.insert(new_path.clone(), file);
            }
        }
        Some(new_path)
    }

    fn get_or_find(&mut self, path: &str) -> GetFindResult {
        if self.files.contains_key(path) {
            GetFindResult::FromCache(self.files.get(path).unwrap())
        } else if let Ok(file_contents) = read_to_string(path) {
            let (file, dia) = File::new(&file_contents, self.line_endings);
            self.files.insert(path.to_string(), file);
            GetFindResult::Parsed(self.files.get(path).unwrap(), if dia.is_empty() {
                None
            } else {
                Some(FileDiagnostic {
                    diagnostics: dia,
                    filename: path.to_string()
                })
            })
        } else {
            GetFindResult::NotFound
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
