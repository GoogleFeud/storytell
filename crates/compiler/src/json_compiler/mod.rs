use storytell_diagnostics::diagnostic::StorytellResult;
use storytell_fs::file_host::FileDiagnostic;
use storytell_parser::ast::model::ASTHeader;
use crate::{base::*, visitors::MagicVariableCollectorContext, path::Path};

pub mod compile;

pub struct JSONCompilerProvider;

impl CompilerProvider for JSONCompilerProvider {
    type Output = String;
    type Context = JSONCompilerContext;

    fn compile_header(_file: &ASTHeader, _ctx: &mut Self::Context) -> StorytellResult<Self::Output> {
        Ok(String::new())
    }
}

pub struct JSONCompilerContext {
    pub magic_variables: MagicVariableCollectorContext,
    pub diagnostics: Vec<FileDiagnostic>,
    pub paths: Path
}

impl CompilerContext for JSONCompilerContext {
    fn add_diagnostic(&mut self, dia: FileDiagnostic) {
        self.diagnostics.push(dia);
    }

    fn get_global_path(&mut self) -> &mut Path {
        &mut self.paths
    }
}

impl JSONCompilerContext {

    pub fn new() -> Self {
        Self { 
            magic_variables: MagicVariableCollectorContext::new(),
            diagnostics: vec![],
            paths: Path::new("global")
        }
    }

}