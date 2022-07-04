use super::file::File;
use std::collections::HashMap;
use std::fs::{read_to_string, write, read_dir};

pub trait FileHost {
    fn create(&mut self, path: &str, content: String) -> bool;
    fn get(&mut self, path: &str) -> Option<&File>;
    fn get_files_from_directory(path: &str) -> Vec<String>;
}

pub struct VirtualFileHost {
    pub files: HashMap<String, File>
}

impl VirtualFileHost {

    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn create_virtual_file(&mut self, path: &str, content: &str) {
        self.files.insert(path.to_string(), File::new(path, content));
    }

}

impl FileHost for VirtualFileHost {
    fn create(&mut self, _path: &str, _content: String) -> bool {
        panic!("Method not allowed for virtual file host")
    }

    fn get(&mut self, path: &str) -> Option<&File> {
        self.files.get(path)
    }

    fn get_files_from_directory(_path: &str) -> Vec<String> {
        panic!("MEthod now allowed for virtual file host.")
    }

}

pub struct SysFileHost {
    pub files: HashMap<String, File>
}

impl SysFileHost {

    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn create_virtual_file(&mut self, path: &str, content: &str) {
        self.files.insert(path.to_string(), File::new(path, content));
    }

}

impl FileHost for SysFileHost {
    fn create(&mut self, path: &str, content: String) -> bool {
       write(path, content).is_ok()
    }

    fn get(&mut self, path: &str) -> Option<&File> {
        if self.files.contains_key(path) {
            self.files.get(path)
        } else {
            if let Ok(file_contents) = read_to_string(path) {
                let file = File::new(path, &file_contents);
                self.files.insert(path.to_string(), file);
                self.files.get(path)
            } else {
                None
            }
        }  
    }

    fn get_files_from_directory(directory: &str) -> Vec<String> {
        let mut files = vec![];
        for thing in read_dir(directory).unwrap() {
            if let Ok(entry) = thing {
                if entry.file_type().unwrap().is_dir() {
                    files.append(&mut Self::get_files_from_directory(entry.path().to_str().unwrap()));
                } else {
                    files.push(entry.path().to_str().unwrap().to_string())
                }
            }
        }
        files
    }

}