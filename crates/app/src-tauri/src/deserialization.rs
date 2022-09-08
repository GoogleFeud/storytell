
use storytell_fs::file_host::{FileDiagnostic, Blob};
use storytell_diagnostics::{diagnostic::Diagnostic, location::Range};
use storytell_compiler::{json};

pub trait JSONCompilable {
    fn compile(&self) -> String;
}

// Due to Rust's orphan rules, serializing structs from other crates becomes
// a big hassle. This is why I'm implementing my own functions.

impl JSONCompilable for Range<usize> {
    fn compile(&self) -> String {
        json!({
            start: self.start,
            end: self.end
        })
    }
}

impl JSONCompilable for Diagnostic {
    fn compile(&self) -> String {
        json!({
            message: self.msg.compile(),
            range: self.range.compile()
        })
    }
}

impl JSONCompilable for FileDiagnostic {
    fn compile(&self) -> String {
        json!({
            filename: self.filename.compile(),
            diagnostics: self.diagnostics.compile()
        })
    }
}

impl<T: JSONCompilable> JSONCompilable for Vec<T> {
    fn compile(&self) -> String {
        format!("[{}]", self.iter().map(|d| d.compile()).collect::<Vec<String>>().join(","))
    }
}

impl JSONCompilable for Blob {
    fn compile(&self) -> String {
        match self {
            Blob::File(filename) => json!({filename: filename.compile()}),
            Blob::Directory(filename, children) => json!({
                filename: filename.compile(),
                children: children.compile()
            })
        }
    }
}

impl JSONCompilable for String {
    fn compile(&self) -> String {
        format!("\"{}\"", self)
    }
}