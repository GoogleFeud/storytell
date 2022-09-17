use storytell_diagnostics::{diagnostic::Diagnostic, location::Range};
use storytell_compiler::{base::files::{File, FileDiagnostic, Directory}, json};
use rustc_hash::FxHashSet;

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
            fileId: self.file_id,
            diagnostics: self.diagnostics.compile()
        })
    }
}

impl<T: JSONCompilable> JSONCompilable for Vec<T> {
    fn compile(&self) -> String {
        format!("[{}]", self.iter().map(|d| d.compile()).collect::<Vec<String>>().join(","))
    }
}

impl JSONCompilable for u16 {
    fn compile(&self) -> String {
        self.to_string()
    }
}

impl JSONCompilable for File {
    fn compile(&self) -> String {
        json!({
            name: self.name.split(".").next().unwrap().compile(),
            id: self.id
        })
    }
}

impl JSONCompilable for Directory {
    fn compile(&self) -> String {
        json!({
            name: self.name.compile(),
            id: self.id,
            children: self.children.compile()
        })
    }
}

impl<T: JSONCompilable> JSONCompilable for FxHashSet<T> {
    fn compile(&self) -> String {
        format!("[{}]", self.iter().map(|d| d.compile()).collect::<Vec<String>>().join(","))
    }
}

impl JSONCompilable for String {
    fn compile(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl JSONCompilable for &str {
    fn compile(&self) -> String {
        format!("\"{}\"", self)
    }
}