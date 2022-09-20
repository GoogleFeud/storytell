use storytell_diagnostics::diagnostic::Diagnostic;
use storytell_fs::FileHost;
use rustc_hash::{FxHashMap, FxHashSet};
use storytell_parser::ast::{model::ASTBlock};
use std::path::{PathBuf, Path};
use std::cell::RefCell;

pub type BlobId = u16;

pub struct FileDiagnostic {
    pub file_id: BlobId,
    pub diagnostics: Vec<Diagnostic>
}

pub struct File {
    pub name: String,
    pub text_content: String,
    pub parsed_content: Vec<ASTBlock>,
    pub path: Vec<BlobId>,
    pub parent: Option<BlobId>,
    pub id: BlobId
}

pub struct Directory {
    pub name: String,
    pub path: Vec<BlobId>,
    pub id: BlobId,
    pub parent: Option<BlobId>,
    pub children: FxHashSet<BlobId>
}

pub enum FileOrDir {
    File(RefCell<File>),
    Directory(RefCell<Directory>)
}

pub struct CompilerFileHost<H: FileHost> {
    pub raw: H,
    pub cwd: String,
    pub files: FxHashMap<BlobId, RefCell<File>>,
    pub dirs: FxHashMap<BlobId, RefCell<Directory>>,
    pub line_endings: usize,
    pub counter: BlobId
}

impl<H: FileHost> CompilerFileHost<H> {

    pub fn new(cwd: &str, line_endings: usize, host: H) -> Self {
        Self {
            raw: host,
            cwd: cwd.to_string(),
            files: FxHashMap::default(),
            dirs: FxHashMap::default(),
            line_endings,
            counter: 1
        }
    }

    pub fn build_path(&self, path: &[BlobId], name: &str) -> PathBuf {
        let mut res = PathBuf::from(&self.cwd);
        for item in path {
            res.push(&self.dirs.get(item).unwrap().borrow().name)
        }
        res.push(name);
        res
    }

    pub fn load_cwd(&mut self) -> FxHashSet<BlobId> {
        let cwd = self.cwd.clone();
        self.load_dir(&cwd, vec![])
    }

    pub fn refresh(&mut self) -> FxHashSet<BlobId> {
        self.counter = 0;
        self.dirs.clear();
        self.files.clear();
        let cwd = self.cwd.clone();
        self.load_dir(&cwd, vec![])
    }

    pub fn load_dir<P: AsRef<Path>>(&mut self, dir: P, path: Vec<BlobId>) -> FxHashSet<BlobId> {
        let mut children: FxHashSet<BlobId> = FxHashSet::default();
        for entry in self.raw.get_entries_from_directory(dir) {
            let blob_id = self.counter;
            children.insert(blob_id);
            self.counter += 1;
            if entry.file_type().unwrap().is_dir() {
                let mut new_path = path.clone();
                new_path.push(blob_id);
                let children = self.load_dir(entry.path(), new_path);
                self.dirs.insert(blob_id,  RefCell::from(Directory {
                    name: entry.file_name().to_str().unwrap().to_string(),
                    path: path.clone(),
                    id: blob_id,
                    parent: path.last().cloned(),
                    children
                }));
            } else {
                self.files.insert(blob_id, RefCell::from(File {
                    parsed_content: vec![],
                    text_content: String::new(),
                    name: entry.file_name().to_str().unwrap().to_string(),
                    path: path.clone(),
                    parent: path.last().cloned(),
                    id: blob_id
                }));
            }
        }
        children
    }
    
    pub fn rename_blob(&mut self, id: &BlobId, name: String) {
        let path = if let Some(file) = self.files.get(id) {
            let mut borrowed = file.borrow_mut();
            let built_path = self.build_path(&borrowed.path, &borrowed.name);
            borrowed.name = name.clone();
            built_path
        } else if let Some(dir) = self.dirs.get(id) {
            let mut borrowed = dir.borrow_mut();
            let built_path = self.build_path(&borrowed.path, &borrowed.name);
            borrowed.name = name.clone();
            built_path
        } else {
            return;
        };
        self.raw.rename_item(path, &name);
    }

    pub fn create_blob(&mut self, name: String, parent: Option<BlobId>, is_dir: bool) -> BlobId {
        let file_id = self.counter;
        self.counter += 1;
        let path = if let Some(id) = &parent {
            let mut folder = self.dirs.get(id).unwrap().borrow_mut();
            folder.children.insert(file_id);
            let mut new_path = folder.path.clone();
            new_path.push(folder.id);
            new_path
        } else { vec![] };
        if is_dir {
            self.raw.create_dir(self.build_path(&path, &name));
            self.dirs.insert(file_id, RefCell::from(Directory {
                name,
                parent,
                path,
                children: FxHashSet::default(),
                id: file_id
            }));
        } else {
            self.raw.write_file(self.build_path(&path, &name), "");
            self.files.insert(file_id, RefCell::from(File {
                name,
                parent,
                path,
                parsed_content: vec![],
                text_content: String::new(),
                id: file_id
            }));
        }
        file_id
    }

    pub fn delete_blob(&mut self, id: &BlobId) {
        match self.delete_blob_in_memory(id) {
            FileOrDir::Directory(dir) => {
                let borrowed = dir.borrow();
                if let Some(parent) = &borrowed.parent {
                    self.dirs.get(parent).unwrap().borrow_mut().children.remove(&borrowed.id);
                }
                self.raw.delete_dir_recursive(self.build_path(&borrowed.path, &borrowed.name));
            }
            FileOrDir::File(file) => {
                let borrowed = file.borrow();
                if let Some(parent) = &borrowed.parent {
                    self.dirs.get(parent).unwrap().borrow_mut().children.remove(&borrowed.id);
                }
                self.raw.delete_file(self.build_path(&borrowed.path, &borrowed.name));
            }
        }
    }

    fn delete_blob_in_memory(&mut self, id: &BlobId) -> FileOrDir {
        if let Some(directory) = self.dirs.remove(id) {
            for child in &directory.borrow().children {
                self.delete_blob_in_memory(child);
            }
            FileOrDir::Directory(directory)
        } else {
            FileOrDir::File(self.files.remove(id).unwrap())
        }
    }

}

