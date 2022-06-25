
pub mod model;
use crate::input::{InputConsumer, ParsingContext};
use model::*;

pub struct Parser<'a, P: ParsingContext> {
    input: InputConsumer<'a, P>
}

impl<'a, P: ParsingContext> Parser<'a, P> {

    pub fn new(text: &'a str, ctx: P) -> Self {
        Self {
            input: InputConsumer::new(text, ctx)
        }
    }

    pub fn parse_block(&mut self) -> Option<ASTBlock> {
        let token = self.input.peek()?;
        let start = self.input.pos;
        match token {
            '#' => {
                self.input.skip();
                let mut depth = 1;
                while let Some(tok) = self.input.peek() {
                    if tok == '#' {
                        depth += 1;
                        self.input.skip();
                    } else {
                        break;
                    }
                }
                Some(ASTBlock::Header(ASTHeader {
                    title: self.input.consume_until_end_of_line().to_string(),
                    depth,
                    attributes: vec![],
                    range: self.input.range_here(start)
                }))
            },
            _ => unimplemented!("Not implemented")
        }
    }
}