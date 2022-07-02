pub mod model;
pub mod utils;
use self::utils::*;
use crate::input::*;
use model::*;
use storytell_diagnostics::{diagnostic::*, make_diagnostics};

make_diagnostics!(define [
    MISSING_CLOSING_SYMBOL,
    1001,
    "Missing closing symbol `$`"
]);

pub enum InlineTextParseResult {
    FoundClosing(ASTText),
    NotFoundOptional(ASTText),
    NotFound,
}

pub struct Parser<'a, P: ParsingContext> {
    input: InputConsumer<'a, P>,
}

impl<'a, P: ParsingContext> Parser<'a, P> {
    pub fn new(text: &'a str, ctx: P) -> Self {
        Self {
            input: InputConsumer::new(text, ctx),
        }
    }

    pub fn parse_block(&mut self, depth: u8) -> Option<ASTBlock> {
        let token = self.input.peek()?;
        let start = self.input.pos;
        match token {
            '#' => {
                self.input.skip();
                let depth = (1 + self.input.count_while('#')) as u8;
                Some(ASTBlock::Header(ASTHeader {
                    title: self.input.consume_until_end_of_line().trim().to_string(),
                    depth,
                    attributes: vec![],
                    range: self.input.range_here(start),
                }))
            }
            '`' if self.input.peek_n(1).is('`') && self.input.peek_n(2).is('`') => {
                self.input.skip_n(3);
                Some(ASTBlock::CodeBlock(ASTCodeBlock {
                    language: self.input.consume_until_end_of_line().to_string(),
                    text: {
                        let code = self.input.consume_until("```")?.to_string();
                        self.input.skip_until_end_of_line();
                        code
                    },
                    attributes: vec![],
                    range: self.input.range_here(start),
                }))
            }
            '@' if self.input.peek_n(1)? == '{' => {
                self.input.skip_n(2);
                Some(ASTBlock::Match(ASTMatch {
                    matched: self.input.consume_until("}")?.to_string(),
                    attributes: vec![],
                    range: self.input.range_here(start),
                    choices: {
                        // Make sure the choice is not on the same line
                        self.input.skip_until_end_of_line();
                        self.parse_choice_list(depth)?.choices
                    },
                    kind: MatchKind::Default,
                }))
            }
            ' ' | '\n' => {
                self.input.skip();
                self.parse_block(depth)
            }
            _ => {
                let paragraph = self.parse_paragraph()?;
                if paragraph.tail.is_empty() && paragraph.parts.is_empty() {
                    None
                } else {
                    Some(ASTBlock::Paragraph(paragraph))
                }
            }
        }
    }

    pub fn parse_choice_list(&mut self, current_depth: u8) -> Option<ASTChoiceGroup> {
        let mut choices: Vec<ASTChoice> = vec![];
        let start = self.input.pos;
        while !self.input.is_eof() {
            println!("{}", self.input.is_on_new_line());
            let ident = self.input.skip_identation();
            println!("{} --- {} --- {:?}", current_depth, ident, self.input.slice(10));
            if current_depth != ident {
                break;
            }
            match self.input.peek()? {
                '-' => {
                    self.input.skip();
                    let start = self.input.pos;
                    choices.push(ASTChoice {
                        text: self.input.consume_until_end_of_line().trim().to_string(),
                        children: self.parse_children(current_depth + 1),
                        attributes: vec![],
                        range: self.input.range_here(start),
                    })
                }
                _ => break,
            }
        }
        Some(ASTChoiceGroup {
            choices,
            range: self.input.range_here(start),
            attributes: vec![],
        })
    }

    pub fn parse_children(&mut self, depth: u8) -> Vec<ASTBlock> {
        let mut res = vec![];
        while !self.input.is_eof() {
            if depth != self.input.skip_identation() {
                break;
            }
            if let Some(block) = self.parse_block(depth) {
                res.push(block);
            } else {
                break;
            }
        }
        res
    }

    pub fn parse_paragraph(&mut self) -> Option<ASTText> {
        match self.parse_text(resolve_line_endings(self.input.ctx.line_endings()), true) {
            InlineTextParseResult::FoundClosing(text)
            | InlineTextParseResult::NotFoundOptional(text) => {
                self.input.skip_n(self.input.ctx.line_endings());
                Some(text)
            }
            InlineTextParseResult::NotFound => None,
        }
    }

    pub fn parse_path_access(&mut self) -> Vec<String> {
        let mut paths: Vec<String> = vec![];
        let mut current_path = String::new();
        while !self.input.is_eof() {
            match self.input.force_next() {
                ch @ '0'..='9' | ch @ 'a'..='z' | ch @ 'A'..='Z' | ch @ '_' => {
                    current_path.push(ch)
                }
                '.' => {
                    paths.push(current_path.clone());
                    current_path.clear()
                }
                _ => break,
            }
        }
        if !current_path.is_empty() {
            paths.push(current_path);
        }
        paths
    }

    pub fn parse_text(&mut self, until: &str, optional: bool) -> InlineTextParseResult {
        let mut parts: Vec<TextPart> = vec![];
        let mut result = String::new();
        let start = self.input.pos;
        let pos_end = self.input.get_pos_of(until).unwrap_or_else(|| {
            if optional {
                self.input.data.len()
            } else {
                0
            }
        });
        if pos_end == 0 {
            InlineTextParseResult::NotFound
        } else {
            while self.input.pos < pos_end {
                match self.input.force_next() {
                    // Escape
                    '\\' => {
                        result.push(self.input.force_next());
                    }
                    // Bold
                    '*' if self.input.peek().is('*') => {
                        let start = self.input.pos - 1;
                        self.input.skip();
                        if let InlineTextParseResult::FoundClosing(text) =
                            self.parse_text("**", false)
                        {
                            self.input.skip_n(2);
                            parts.push(TextPart {
                                before: result.clone(),
                                text: ASTInline {
                                    kind: ASTInlineKind::Bold(text),
                                    range: self.input.range_here(start),
                                    attributes: vec![],
                                },
                            });
                            result.clear()
                        } else {
                            result.push_str("**");
                        }
                    }
                    // Italics
                    '*' => {
                        let start = self.input.pos - 1;
                        if let InlineTextParseResult::FoundClosing(text) =
                            self.parse_text("*", false)
                        {
                            self.input.skip();
                            parts.push(TextPart {
                                before: result.clone(),
                                text: ASTInline {
                                    kind: ASTInlineKind::Italics(text),
                                    range: self.input.range_here(start),
                                    attributes: vec![],
                                },
                            });
                            result.clear()
                        } else {
                            result.push('*');
                        }
                    }
                    // Divert
                    '-' if self.input.peek().is('>') => {
                        self.input.skip();
                        if self.input.peek().is(' ') {
                            self.input.skip()
                        }
                        let text = self.parse_path_access();
                        parts.push(TextPart {
                            before: result.clone(),
                            text: ASTInline {
                                kind: ASTInlineKind::Divert(text),
                                attributes: vec![],
                                range: self.input.range_here(start),
                            },
                        });
                        result.clear()
                    }
                    other => result.push(other),
                }
            }
            InlineTextParseResult::FoundClosing(ASTText {
                parts,
                tail: result,
                attributes: vec![],
                range: self.input.range_here(start),
            })
        }
    }

    pub fn parse(&mut self) -> Vec<ASTBlock> {
        let mut res = vec![];
        while !self.input.is_eof() {
            if let Some(block) = self.parse_block(0) {
                res.push(block);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use storytell_diagnostics::diagnostic::Diagnostic;

    use super::*;

    pub struct Context {
        pub errors: Vec<Diagnostic>,
    }

    impl ParsingContext for Context {
        fn line_endings(&self) -> usize {
            1
        }

        fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
            self.errors.push(diagnostic);
        }
    }

    impl Context {
        pub fn new() -> Self {
            Self { errors: vec![] }
        }
    }

    #[test]
    fn parse_header() {
        let mut input = Parser::new("# This is some header!!!...\n", Context::new());
        let header = input.parse_block(0);
        assert!(matches!(header, Some(ASTBlock::Header(_)),));
        if let Some(ASTBlock::Header(block)) = header {
            assert_eq!(block.title, "This is some header!!!...");
            assert_eq!(block.depth, 1);
        }
    }

    #[test]
    fn parse_codeblock() {
        let mut input = Parser::new(
            "# This is some header!!!...\n```js\nThis is a code\nblock...\nyeah...```",
            Context::new(),
        );
        input.parse_block(0);
        let code_block = input.parse_block(0);
        assert!(matches!(code_block, Some(ASTBlock::CodeBlock(_))));
        if let Some(ASTBlock::CodeBlock(block)) = code_block {
            assert_eq!(block.text, "This is a code\nblock...\nyeah...");
            assert_eq!(block.language, "js");
        } else {
            panic!("Code block")
        }
    }

    #[test]
    fn parse_inline_bold() {
        let mut input = Parser::new("# This is some header!!!...\nThis is **a** paragraph, pretty cool... **really** cool! Same paragraph...\nAlright this is a different one!!## Another heading", Context::new());
        input.parse_block(0); // Header
        let paragraph = input.parse_paragraph().unwrap();
        assert_eq!(
            "This is a paragraph, pretty cool... really cool! Same paragraph...",
            paragraph.to_raw()
        );
    }

    #[test]
    fn parse_inline_italics() {
        let mut input = Parser::new(
            "# This is some header!!!...\n**really** interesting *word*...\nAlright",
            Context::new(),
        );
        input.parse_block(0); // Header
        let paragraph = input.parse_paragraph().unwrap();
        assert_eq!("really interesting word...", paragraph.to_raw());
        assert!(matches!(
            paragraph.parts[1].text.kind,
            ASTInlineKind::Italics(_)
        ));
    }

    #[test]
    fn parse_inline_divert() {
        let mut input = Parser::new("# This is some header!!!...\n**really** interesting *word...\nAlright, second paragraph -> second_chapter", Context::new());
        input.parse_block(0); // Header
        input.parse_paragraph();
        let second_para = input.parse_paragraph().unwrap();
        if let ASTInlineKind::Divert(arrow) = &second_para.parts[0].text.kind {
            assert_eq!(arrow[0], "second_chapter")
        } else {
            panic!("Divert")
        }
    }

    #[test]
    fn parse_choice_list() {
        let input = Parser::new(
            "
# This is a chapter

This is a **paragraph**...

@{match_condition}
- {true}
    ```js
    This is a language!
    ```
    So yeah, what's up?
    Wohoooo!
    @{nested_match}
    - {a == 1}
    - {}
- {false}
    The option is false!
- Third option...
",
            Context::new(),
        )
        .parse();
        //println!("{:?}", input);
        if let ASTBlock::Match(matcher) = &input[2] {
            assert_eq!(matcher.matched, "match_condition");
            assert_eq!(matcher.choices.len(), 3);
            println!("{:?}", matcher.choices);
            /*
            if let ASTBlock::Match(nested_match) = &matcher.choices[0].children[3] {
                assert_eq!(nested_match.choices.len(), 2);
            } else {
                panic!("Nested match")
            }
            */
        } else {
            panic!("Match")
        }
    }
}
