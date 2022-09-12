use std::{fmt::Display, path::PathBuf};
use storytell_fs::{file::{File, Directory}, file_host::{FileDiagnostic}};
use storytell_diagnostics::{diagnostic::Diagnostic, location::Range};
use storytell_compiler::{json};
use rustc_hash::FxHashMap;

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
            name: PathBuf::from(&self.path).iter().last().unwrap().to_str().unwrap().split('.').next().unwrap().to_string().compile(),
            id: self.id
        })
    }
}

impl JSONCompilable for Directory {
    fn compile(&self) -> String {
        json!({
            name: PathBuf::from(&self.path).iter().last().unwrap().to_str().unwrap().to_string().compile(),
            children: self.children.compile(),
            id: self.id
        })
    }
}

impl<K: Display, V: JSONCompilable> JSONCompilable for FxHashMap<K, V> {
    fn compile(&self) -> String {
        format!("{{{}}}", self.iter().map(|(k, v)| format!("\"{}\":{}", k, v.compile())).collect::<Vec<String>>().join(","))
    }
}

impl JSONCompilable for String {
    fn compile(&self) -> String {
        format!("\"{}\"", self)
    }
}