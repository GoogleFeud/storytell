
use crate::tokenizer::Tokenizer;
pub mod ast;

pub struct JsParser<'a> {
    pub tokens: Tokenizer<'a>
}

impl<'a> JsParser<'a> {

    pub fn new(content: &'a str) -> Self {
        Self { 
            tokens: Tokenizer::new(content)
        }
    }

    
}


