use storytell_diagnostics::diagnostic::StorytellResult;
use storytell_parser::ast::model::*;
use std::stringify;

use crate::json_compiler::JSONCompilerContext;

macro_rules! strip_plus {
    (+ $($rest: tt)*) => {
        $($rest)*
    }
}

macro_rules! json {
    {$($property: ident: $value: expr),+} => {
        strip_plus!($(
            +format!("\"{}\":{}", stringify!($property), $value.safe_compile())
        )+)
    },
    ($value: expr) => {
        $value.safe_compile()
    }
}

pub trait JSONCompilable {
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String>;
}

pub trait JSONSafeCompilable {
    fn safe_compile(&self) -> String;
}


impl JSONSafeCompilable for String {
    fn safe_compile(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl<T: JSONSafeCompilable> JSONSafeCompilable for Vec<T> {
    fn safe_compile(&self) -> String {
        format!("[{}]", self.iter().map(|i| i.safe_compile()).collect::<Vec<String>>().join(","))
    }
}

impl JSONSafeCompilable for u32 {
    fn safe_compile(&self) -> String {
        self.to_string()
    }
}

impl JSONSafeCompilable for i32 {
    fn safe_compile(&self) -> String {
        self.to_string()
    }
}

impl JSONSafeCompilable for &str {
    fn safe_compile(&self) -> String {
        format!("\"{}\"", self)
    }
}