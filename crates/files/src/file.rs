use std::fs::read_to_string;
use storytell_diagnostics::diagnostic::Diagnostic;
use storytell_parser::{ast::{model::ASTBlock, Parser}, input::ParsingContext};

pub struct File {
    pub id: u16,
    pub path: String,
    pub content: Vec<ASTBlock>
}

impl File {

    pub fn empty(id: u16, path: &str) -> Self {
        Self {
            id,
            path: path.to_string(),
            content: vec![]
        }
    }

    pub fn parse(&mut self, line_endings: usize) -> Vec<Diagnostic> {
        let (res, ctx) = Parser::new(&read_to_string(&self.path).unwrap(), ParsingContext::new(line_endings)).parse();
        self.content = res;
        ctx.diagnostics
    }

    pub fn new(id: u16, path: &str, content: &str, line_endings: usize) -> (Self, Vec<Diagnostic>) {
        let (res, ctx) = Parser::new(content, ParsingContext::new(line_endings)).parse();
        (Self {
            id,
            path: path.to_string(),
            content: res
        }, ctx.diagnostics)
    }

    pub fn reparse(&mut self, content: &str, line_endings: usize) -> Vec<Diagnostic> {
        let (res, ctx) = Parser::new(content, ParsingContext::new(line_endings)).parse();
        self.content = res;
        ctx.diagnostics
    }
    
}

pub struct Directory {
    pub id: u16,
    pub path: String,
    pub children: Vec<u16>
}
