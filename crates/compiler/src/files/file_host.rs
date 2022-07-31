use super::file::File;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, write};

pub trait FileHost {
    fn create(&mut self, path: &str, content: String) -> bool;
    fn get(&mut self, path: &str) -> Option<&File>;
    fn get_files_from_directory(&self, path: &str) -> Vec<String>;
}

pub struct VirtualFileHost {
    pub files: HashMap<String, File>,
}

impl VirtualFileHost {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}

impl Default for VirtualFileHost {
    fn default() -> Self {
        Self::new()
    }
}

impl FileHost for VirtualFileHost {
    fn create(&mut self, path: &str, content: String) -> bool {
        let file = File::new(path, &content);
        self.files.insert(path.to_string(), file);
        true
    }

    fn get(&mut self, path: &str) -> Option<&File> {
        self.files.get(path)
    }

    fn get_files_from_directory(&self, _path: &str) -> Vec<String> {
        panic!("Method now allowed for virtual file host.")
    }
}

pub struct SysFileHost {
    pub files: HashMap<String, File>,
}

impl SysFileHost {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}

impl Default for SysFileHost {
    fn default() -> Self {
        Self::new()
    }
}

impl FileHost for SysFileHost {
    fn create(&mut self, path: &str, content: String) -> bool {
        write(path, content).is_ok()
    }

    fn get(&mut self, path: &str) -> Option<&File> {
        if self.files.contains_key(path) {
            self.files.get(path)
        } else if let Ok(file_contents) = read_to_string(path) {
            let file = File::new(path, &file_contents);
            self.files.insert(path.to_string(), file);
            self.files.get(path)
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

}
