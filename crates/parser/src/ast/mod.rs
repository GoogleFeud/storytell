
pub mod model;
pub mod utils;
use crate::input::{InputConsumer, ParsingContext};
use model::*;

use self::utils::resolve_line_endings;

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
                    language: self.input.consume_until_end_of_line().to_string(),
                    text: self.input.consume_until("```")?.to_string(),
                    attributes: vec![],
                    range: self.input.range_here(start)
                }))
            },
            '@' if self.input.peek_n(1)? == '{' => {
                self.input.skip_n(2);
                None
            },
            _ => unimplemented!("Not implemented")
        }
    }

    pub fn parse_choice_list(&mut self) -> Option<ASTChoiceGroup> {
        //let mut choices: Vec<ASTChoice> = vec![];
        //let start = self.input.pos;
        unimplemented!("Not implemented")
    }

    pub fn parse_paragraph(&mut self) -> Option<ASTText> {
        self.parse_text(resolve_line_endings(self.input.ctx.line_endings()))
    }

    pub fn parse_text(&mut self, until: &str) -> Option<ASTText> {
        let mut parts: Vec<TextPart> = vec![];
        let mut result = String::new();
        let start = self.input.pos;
        while !self.input.is_eof() {
            if self.input.slice(until.len()) == until {
                self.input.skip_n(until.len());
                break;
            } else {
                match self.input.force_next() {
                    '*' if self.input.peek()? == '*' => {
                        let start = self.input.pos - 1;
                        self.input.skip();
                        parts.push(TextPart { before: result.clone(), text: ASTInline {
                            kind: ASTInlineKind::Bold,
                            text: self.parse_text("**")?,
                            range: self.input.range_here(start),
                            attributes: vec![]
                        }});
                        result.clear()
                    }
                    other => result.push(other)
                }
            }
        }
        Some(ASTText {
            parts,
            tail: result,
            range: self.input.range_here(start),
            attributes: vec![]
        })
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
        let mut input = Parser::new("# This is some header!!!...\n```js\nThis is a code\nblock...\nyeah...```", Context {});
        input.parse_block();
        let code_block = input.parse_block();
        assert!(matches!(code_block, Some(ASTBlock::CodeBlock(_))));
        if let Some(ASTBlock::CodeBlock(block)) = code_block {
            assert_eq!(block.text, "This is a code\nblock...\nyeah...");
            assert_eq!(block.language, "js");
        }
    }

    #[test]
    fn parse_inline_bold() {
        let mut input = Parser::new("# This is some header!!!...\nThis is a paragraph, pretty cool... **really** cool!", Context {});
        input.parse_block(); // Header
        let paragraph = input.parse_paragraph();
        assert!(matches!(paragraph, Some(_)));
        if let Some(text) = paragraph {
            println!("TESSST: {:?}", text);
            assert_eq!("This is a paragraph, pretty cool... really cool!", text.to_raw());
        }
    }

}