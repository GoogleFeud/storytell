use storytell_diagnostics::diagnostic::Diagnostic;
use storytell_parser::{ast::{model::ASTBlock, Parser}, input::ParsingContext};

pub struct File {
    pub id: u16,
    pub name: String,
    pub path: Vec<u16>,
    pub content: Vec<ASTBlock>
}

impl File {

    pub fn empty(id: u16, path: Vec<u16>, name: String) -> Self {
        Self {
            id,
            name,
            path,
            content: vec![]
        }
    }

    pub fn parse(&mut self, content: &str, line_endings: usize) -> Vec<Diagnostic> {
        let (res, ctx) = Parser::new(content, ParsingContext::new(line_endings)).parse();
        self.content = res;
        ctx.diagnostics
    }
    
}

pub struct Directory {
    pub id: u16,
    pub name: String,
    pub path: Vec<u16>,
    pub children: Vec<u16>
}

pub enum Blob {
    File(File),
    Folder(Directory)
}