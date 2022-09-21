use storytell_diagnostics::{diagnostic::Diagnostic, location::Range};
use storytell_compiler::{base::files::{File, FileDiagnostic, Directory}, json};
use rustc_hash::FxHashSet;
use serde_json::{json as serde_json};

pub trait JSONSerializable {
    fn compile(&self) -> String;
}

// Due to Rust's orphan rules, serializing structs from other crates becomes
// a big hassle. This is why I'm implementing my own functions.

impl JSONSerializable for Range<usize> {
    fn compile(&self) -> String {
        json!({
            start: self.start,
            end: self.end
        })
    }
}

impl JSONSerializable for Diagnostic {
    fn compile(&self) -> String {
        json!({
            message: self.msg.compile(),
            range: self.range.compile()
        })
    }
}

impl JSONSerializable for FileDiagnostic {
    fn compile(&self) -> String {
        if self.diagnostics.is_empty() {
            String::from("null")
        } else {
            self.diagnostics.compile()
        }
    }
}

impl<T: JSONSerializable> JSONSerializable for Vec<T> {
    fn compile(&self) -> String {
        format!("[{}]", self.iter().map(|d| d.compile()).collect::<Vec<String>>().join(","))
    }
}

impl JSONSerializable for u16 {
    fn compile(&self) -> String {
        self.to_string()
    }
}

impl JSONSerializable for File {
    fn compile(&self) -> String {
        json!({
            name: self.name.split(".").next().unwrap().compile(),
            parent: self.parent.compile(),
            id: self.id
        })
    }
}

impl JSONSerializable for Directory {
    fn compile(&self) -> String {
        json!({
            name: self.name.compile(),
            id: self.id,
            children: self.children.compile(),
            parent: self.parent.compile()
        })
    }
}

impl<T: JSONSerializable> JSONSerializable for FxHashSet<T> {
    fn compile(&self) -> String {
        format!("[{}]", self.iter().map(|d| d.compile()).collect::<Vec<String>>().join(","))
    }
}

impl JSONSerializable for String {
    fn compile(&self) -> String {
        serde_json!(self).to_string()
    }
}

impl JSONSerializable for &str {
    fn compile(&self) -> String {
        serde_json!(self).to_string()
    }
}

impl<T: JSONSerializable> JSONSerializable for Option<T> {
    fn compile(&self) -> String {
        if let Some(val) = &self {
            val.compile()
        } else {
            String::from("null")
        }
    }
}