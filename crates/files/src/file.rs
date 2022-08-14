use storytell_diagnostics::diagnostic::Diagnostic;
use storytell_parser::{ast::{model::ASTBlock, Parser}, input::ParsingContext};

pub struct File {
    pub content: Vec<ASTBlock>,
    pub path: String
}

impl File {
    
    pub fn new(path: &str, content: &str) -> (Self, Vec<Diagnostic>) {
        let (res, ctx) = Parser::new(content, ParsingContext::new(if cfg!(target_os = "windows") {
            2
        } else {
            1
        })).parse();
        (Self { 
            content: res,
            path: path.to_string()
        }, ctx.diagnostics)
    }

    pub fn reparse(&mut self, content: &str) -> Vec<Diagnostic> {
        let (res, ctx) = Parser::new(content, ParsingContext::new(if cfg!(target_os = "windows") {
            2
        } else {
            1
        })).parse();
        self.content = res;
        ctx.diagnostics
    }
    
}