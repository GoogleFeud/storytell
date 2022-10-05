use storytell_compiler::{base::{Compiler, files::BlobId}, json_compiler::{JSONCompilerProvider, JSONCompilerContext}, visitors::variable_collector::variable::VariableContainer};
use storytell_fs::SysFileHost;

pub struct CompilerWrapper {
    pub inner: Compiler<JSONCompilerProvider, SysFileHost>,
    pub variables: VariableContainer<BlobId>
}

impl CompilerWrapper {

    pub fn new(cwd: &str, line_endings: usize, host: SysFileHost) -> Self {
        Self {
            inner: Compiler::new(cwd, line_endings, host),
            variables: VariableContainer::default()
        }
    }

    pub fn create_ctx() -> JSONCompilerContext {
        JSONCompilerContext::new(Some("this".to_string()))
    }

}