use storytell_diagnostics::diagnostic::Diagnostic;
use storytell_parser::{ast::{model::ASTBlock, Parser}, input::ParsingContext};

pub struct ParsingCtx {
    pub diagnostics: Vec<Diagnostic>
}

impl ParsingCtx {
    pub fn new() -> Self {
        Self { diagnostics: vec![] }
    }
}

impl ParsingContext for ParsingCtx {
    fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn line_endings(&self) -> usize {
        if cfg!(target_os = "windows") {
            2
        } else {
            1
        }
    }
}

pub struct File {
    pub content: Vec<ASTBlock>,
    pub path: String
}

impl File {
    
    pub fn new(path: &str, content: &str) -> Self {
        let parsed = Parser::new(content, ParsingCtx::new()).parse();
        Self { 
            content: parsed.0,
            path: path.to_string()
         }
    }

    pub fn reparse(&mut self, content: &str) {
        self.content = Parser::new(content, ParsingCtx::new()).parse().0;
    }
    
}