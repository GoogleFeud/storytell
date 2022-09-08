use storytell_diagnostics::diagnostic::Diagnostic;
use storytell_parser::{ast::{model::ASTBlock, Parser}, input::ParsingContext};

pub struct File {
    pub content: Vec<ASTBlock>,
    pub path: String
}

impl File {

    pub fn empty(path: &str) -> Self {
        Self { 
            content: vec![],
            path: path.to_string()
        }
    }
    
    pub fn new(path: &str, content: &str, line_endings: usize) -> (Self, Vec<Diagnostic>) {
        let (res, ctx) = Parser::new(content, ParsingContext::new(line_endings)).parse();
        (Self { 
            content: res,
            path: path.to_string()
        }, ctx.diagnostics)
    }

    pub fn reparse(&mut self, content: &str, line_endings: usize) -> Vec<Diagnostic> {
        let (res, ctx) = Parser::new(content, ParsingContext::new(line_endings)).parse();
        self.content = res;
        ctx.diagnostics
    }
    
}