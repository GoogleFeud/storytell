
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
                let depth = (1 + self.input.count_while('#')) as u8;
                if self.input.peek()? == ' ' {
                    self.input.skip();
                }
                Some(ASTBlock::Header(ASTHeader {
                    title: self.input.consume_until_end_of_line().to_string(),
                    depth,
                    attributes: vec![],
                    range: self.input.range_here(start)
                }))
            },
            '`' if self.input.peek_n(1)? == '`' && self.input.peek_n(2)? == '`' => {
                self.input.skip_n(3);
                Some(ASTBlock::CodeBlock(ASTCodeBlock {
                    text: self.input.consume_until("```")?.to_string(),
                    language: String::from("js"),
                    attributes: vec![],
                    range: self.input.range_here(start)
                }))
            }
            _ => unimplemented!("Not implemented")
        }
    }
}


#[cfg(test)]
mod tests {
    use storytell_diagnostics::diagnostic::Diagnostic;

    use super::*;

    pub struct Context {}

    impl ParsingContext for Context {
        fn line_endings(&self) -> usize {
            1
        }

        fn add_diagnostic(&mut self, _diagnostic: Diagnostic) {
            unimplemented!("Not necessary")
        }
    }

    #[test]
    fn parse_header() {
        let mut input = Parser::new("# This is some header!!!...\n", Context {});
        let header = input.parse_block();
        assert!(matches!(header, Some(ASTBlock::Header(_)),));
        if let Some(ASTBlock::Header(block)) = header {
            assert_eq!(block.title, "This is some header!!!...");
            assert_eq!(block.depth, 1);
        }
    }

    #[test]
    fn parse_codeblock() {
        let mut input = Parser::new("# This is some header!!!...\n```This is a code\nblock...\nyeah...```", Context {});
        input.parse_block();
        let code_block = input.parse_block();
        assert!(matches!(code_block, Some(ASTBlock::CodeBlock(_)),));
        if let Some(ASTBlock::CodeBlock(block)) = code_block {
            assert_eq!(block.text, "This is a code\nblock...\nyeah...");
        }
    }

}