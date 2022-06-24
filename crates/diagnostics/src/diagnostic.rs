use crate::location::Range;
use std::fmt::{ Debug, Formatter, Display, Result as FmtResult };

pub struct DiagnosticMessage {
    pub code: u16,
    pub message: &'static str
}

pub fn format_diagnostic(diagnostic: &DiagnosticMessage, vars: Vec<&str>) -> String {
    let msg = diagnostic.message;
    if vars.is_empty() { return msg.to_string() };
    let mut ind: usize = 0;
    let mut new_str = String::new();
    for ch in msg.chars() {
        if ch == '$' {
            new_str.push_str(vars[ind]);
            ind += 1;
        } else {
            new_str.push(ch)
        }
    }
    new_str
}

pub enum DiagnosticVariants {
    Suggestion,
    Warning,
    Error
}

pub struct Diagnostic {
    pub range: Range,
    pub msg: String,
    pub variant: DiagnosticVariants
}

impl Debug for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Error")
         .field("msg", &self.msg.to_string())
         .field("range", &self.range.to_string())
         .finish()
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Error: {}", self.msg)
    }
}

pub type StroytellResult<T> = Result<T, Diagnostic>;

pub trait DiagnosticCollector {
    fn add_diagnostic(&mut self, err: Diagnostic);
}

#[macro_export]
macro_rules! make_diagnostics {
    ($([$name:ident, $code:expr, $msg:expr]),+) => {
        impl Diagnostics {
            $(
                pub const $name: DiagnosticMessage = DiagnosticMessage {
                    code: $code,
                    message: $msg
                };
            )+
        }
    };
    (define $([$name:ident, $code:expr, $msg:expr]),+) => {
        pub struct Diagnostics;
        impl Diagnostics {
            $(
                pub const $name: DiagnosticMessage = DiagnosticMessage {
                    code: $code,
                    message: $msg
                };
            )+
        }
    };
}