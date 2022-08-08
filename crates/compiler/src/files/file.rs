use storytell_parser::{ast::{model::ASTBlock, Parser}, input::ParsingContext};

pub struct File {
    pub content: Vec<ASTBlock>,
    pub path: String
}

impl File {
    
    pub fn new(path: &str, content: &str) -> Self {
        let parsed = Parser::new(content, ParsingContext::new(if cfg!(target_os = "windows") {
            2
        } else {
            1
        })).parse();
        Self { 
            content: parsed.0,
            path: path.to_string()
         }
    }

    pub fn reparse(&mut self, content: &str) {
        self.content = Parser::new(content, ParsingContext::new(if cfg!(target_os = "windows") {
            2
        } else {
            1
        })).parse().0;
    }
    
}