use storytell_diagnostics::diagnostic::Diagnostic;
use storytell_fs::FileHost;
use rustc_hash::FxHashMap;
use storytell_parser::{ast::{model::ASTBlock, Parser}, input::ParsingContext};
use std::{path::{PathBuf, Path}, cell::RefMut};
use std::cell::RefCell;

pub type BlobId = u16;

pub struct FileDiagnostic {
    pub file_id: BlobId,
    pub diagnostics: Vec<Diagnostic>
}

pub struct File {
    pub name: String,
    pub parsed_content: Vec<ASTBlock>,
    pub path: Vec<BlobId>,
    pub id: BlobId
}

pub struct Directory {
    pub name: String,
    pub path: Vec<BlobId>,
    pub id: BlobId,
    pub children: Vec<BlobId>
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
            counter: 0
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

    pub fn load_cwd(&mut self) -> Vec<BlobId> {
        let cwd = self.cwd.clone();
        self.load_dir(&cwd, vec![])
    }

    pub fn load_dir<P: AsRef<Path>>(&mut self, dir: P, path: Vec<BlobId>) -> Vec<BlobId> {
        let mut children: Vec<BlobId> = vec![];
        for entry in self.raw.get_entries_from_directory(dir) {
            let blob_id = self.counter;
            children.push(blob_id);
            self.counter += 1;
            if entry.file_type().unwrap().is_dir() {
                let mut new_path = path.clone();
                new_path.push(blob_id);
                let children = self.load_dir(entry.path(), new_path);
                self.dirs.insert(blob_id,  RefCell::from(Directory {
                    name: entry.file_name().to_str().unwrap().to_string(),
                    path: path.clone(),
                    id: blob_id,
                    children
                }));
            } else {
                self.files.insert(blob_id, RefCell::from(File {
                    parsed_content: vec![],
                    name: entry.file_name().to_str().unwrap().to_string(),
                    path: path.clone(),
                    id: blob_id
                }));
            }
        }
        children
    }

    pub fn parse_file_by_id(&self, id: &BlobId) -> Option<(RefMut<File>, Vec<Diagnostic>)> {
        let mut file = self.files.get(id).unwrap().borrow_mut();
        let file_content = self.raw.read_file(self.build_path(&file.path, &file.name))?;
        let (res, ctx) = Parser::new(&file_content, ParsingContext::new(self.line_endings)).parse();
        file.parsed_content = res;
        Some((file, ctx.diagnostics))
    }

    pub fn parse_file(&self, file: &File) -> Option<(Vec<ASTBlock>, Vec<Diagnostic>)> {
        let file_content = self.raw.read_file(self.build_path(&file.path, &file.name))?;
        let (res, ctx) = Parser::new(&file_content, ParsingContext::new(self.line_endings)).parse();
        Some((res, ctx.diagnostics))
    }
    
    pub fn rename_blob(&mut self, id: &BlobId, name: String) {
        let path = if let Some(file) = self.files.get(id) {
            let mut borrowed = file.borrow_mut();
            let built_path =             self.build_path(&borrowed.path, &borrowed.name);
            borrowed.name = name.clone();
            built_path
        } else if let Some(dir) = self.dirs.get(id) {
            let mut borrowed = dir.borrow_mut();
            let built_path =             self.build_path(&borrowed.path, &borrowed.name);
            borrowed.name = name.clone();
            built_path
        } else {
            return;
        };
        self.raw.rename_item(path, &name);
    }

    pub fn delete_blob(&mut self, id: &BlobId) {
        match self.delete_blob_in_memory(id) {
            FileOrDir::Directory(dir) => {
                let borrowed = dir.borrow();
                self.raw.delete_dir_recursive(self.build_path(&borrowed.path, &borrowed.name));
            }
            FileOrDir::File(file) => {
                let borrowed = file.borrow();
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

