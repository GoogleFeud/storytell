
use std::{fs::{read_dir, write, rename, read_to_string, remove_file, remove_dir_all, ReadDir}, path::Path, iter::Flatten};

pub trait FileHost {
    fn write_file<P: AsRef<Path>>(&mut self, path: P, content: &str) -> Option<()>;
    fn rename_item<P: AsRef<Path>>(&mut self, path: P, name: &str) -> Option<(String, bool)>;
    fn delete_file<P: AsRef<Path>>(&mut self, path: P) -> Option<()>;
    fn delete_dir<P: AsRef<Path>>(&mut self, path: P) -> Option<()>;
    fn read_file<P: AsRef<Path>>(&self, path: P) -> Option<String>;
    fn get_entries_from_directory<P: AsRef<Path>>(&self, path: P) -> Flatten<ReadDir>;
    fn get_files_from_directory<P: AsRef<Path>>(&self, path: P) -> Vec<String>;
}

#[derive(Default)]
pub struct SysFileHost;

impl FileHost for SysFileHost {

    fn write_file<P: AsRef<Path>>(&mut self, path: P, content: &str) -> Option<()> {
        write(path, content).ok()
    }

    fn rename_item<P: AsRef<Path>>(&mut self, path: P, name: &str) -> Option<(String, bool)> {
        let old_path = path.as_ref();
        let new_path = old_path.parent()?.join(name).to_str()?.to_string();
        rename(&old_path, &new_path).ok()?;
        Some((new_path, old_path.is_dir()))
    }

    fn delete_file<P: AsRef<Path>>(&mut self, path: P) -> Option<()> {
        remove_file(path).ok()
    }

    fn delete_dir<P: AsRef<Path>>(&mut self, path: P) -> Option<()> {
        remove_dir_all(path).ok()
    }

    fn read_file<P: AsRef<Path>>(&self, path: P) -> Option<String> {
        read_to_string(path).ok()
    }

    fn get_entries_from_directory<P: AsRef<Path>>(&self, path: P) -> Flatten<ReadDir> {
        read_dir(path).unwrap().flatten()
    }

    fn get_files_from_directory<P: AsRef<Path>>(&self, directory: P) -> Vec<String> {
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
