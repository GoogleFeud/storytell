use storytell_diagnostics::diagnostic::StorytellResult;
use storytell_parser::ast::model::*;

use crate::json_compiler::JSONCompilerContext;

pub trait JSONCompilable {
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String>;
}

pub trait JSONSafeCompilable {
    fn safe_compile(&self) -> String;
}
