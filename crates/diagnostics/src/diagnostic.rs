use crate::location::Range;
use std::fmt::{ Debug, Formatter, Display, Result as FmtResult };

pub struct DiagnosticMessage {
    pub code: &'static str,
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

#[derive(Clone, Debug, PartialEq)]
pub enum DiagnosticVariants {
    Suggestion,
    Warning,
    Error
}

#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub range: Range<usize>,
    pub msg: String,
    pub variant: DiagnosticVariants
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Error: {}", self.msg)
    }
}

pub type StorytellResult<T> = Result<T, Vec<Diagnostic>>;

pub trait DiagnosticCollector {
    fn add_diagnostic(&mut self, err: Diagnostic);
}

#[macro_export]
macro_rules! make_diagnostics {
    ($([$name:ident, $code:expr, $msg:expr]),+) => {
        impl Diagnostics {
            $(
                pub const $name: DiagnosticMessage = DiagnosticMessage {
                    code: stringify!($code),
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
                    code: stringify!($code),
                    message: $msg
                };
            )+
        }
    };
}

#[macro_export]
macro_rules! dia {
    ($diagnostic: ident, $range: expr) => {
        Diagnostic {
            msg: format_diagnostic(&Diagnostics::$diagnostic, vec![]),
            range: $range,
            variant: DiagnosticVariants::Error
        }
    };
    ($diagnostic: ident, $range: expr, $($vars: expr),*) => {
        Diagnostic {
            msg: format_diagnostic(&Diagnostics::$diagnostic, vec![$($vars),*]),
            range: $range,
            variant: DiagnosticVariants::Error
        }
    };
    ($diagnostic: ident, $range: expr, $variant: ident, $($vars: expr),*) => {
        Diagnostic {
            msg: format_diagnostic(&Diagnostics::$diagnostic, vec![$($vars),*]),
            range: $range,
            variant: DiagnosticVariants::$variant
        }
    }
}