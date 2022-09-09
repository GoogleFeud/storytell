use storytell_diagnostics::diagnostic::Diagnostic;
use storytell_parser::{ast::{model::ASTBlock, Parser}, input::ParsingContext};

pub struct File {
    pub content: Vec<ASTBlock>
}

impl File {

    pub fn empty() -> Self {
        Self { 
            content: vec![]
        }
    }
    
    pub fn new(content: &str, line_endings: usize) -> (Self, Vec<Diagnostic>) {
        let (res, ctx) = Parser::new(content, ParsingContext::new(line_endings)).parse();
        (Self { 
            content: res
        }, ctx.diagnostics)
    }

    pub fn reparse(&mut self, content: &str, line_endings: usize) -> Vec<Diagnostic> {
        let (res, ctx) = Parser::new(content, ParsingContext::new(line_endings)).parse();
        self.content = res;
        ctx.diagnostics
    }
    
}