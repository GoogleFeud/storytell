pub mod model;
pub mod utils;

use self::utils::*;
use crate::input::*;
use model::*;
use storytell_diagnostics::{diagnostic::*, make_diagnostics, dia};

make_diagnostics!(define [
    REQUIRED_JS,
    1001,
    "Match condition must be a javascript inline expression."
], [
    MISSING_CLOSING,
    1002,
    "Missing closing character '$'."
]);

pub enum InlineTextParseResult {
    FoundClosing(ASTText),
    NotFoundOptional(ASTText),
    NotFound,
}

pub struct Parser<'a, P: ParsingContext> {
    input: InputConsumer<'a, P>,
    collected_attributes: VecStack<ASTAttribute>
}

impl<'a, P: ParsingContext> Parser<'a, P> {
    pub fn new(text: &'a str, ctx: P) -> Self {
        Self {
            input: InputConsumer::new(text, ctx),
            collected_attributes: VecStack::new()
        }
    }

    pub fn parse_block(&mut self, depth: u8) -> Option<ASTBlock> {
        let token = self.input.peek()?;
        let start = self.input.pos;
        match token {
            '#' => {
                self.input.skip();
                if self.input.peek().is('[') {
                    self.input.skip();
                    let attrs = self.parse_attributes();
                    self.collected_attributes.push_vec(attrs);
                    return self.parse_block(depth);
                }
                let depth = (1 + self.input.count_while('#')) as u8;
                Some(ASTBlock::Header(ASTHeader {
                    title: self.input.consume_until_end_of_line().trim().to_string(),
                    depth,
                    attributes: self.collected_attributes.pop_vec(),
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
                    attributes: self.collected_attributes.pop_vec(),
                    range: self.input.range_here(start),
                }))
            }
            '@' if self.input.peek_n(1).is('{') => {
                self.input.skip_n(2);
                let kind = if self.input.peek().is(':') {
                    self.input.skip();
                    match self.input.consume_until(" ") {
                        Some("if") => MatchKind::If,
                        Some("not") => MatchKind::Not,
                        _ => MatchKind::Default
                    }
                } else { MatchKind::Default };
                Some(ASTBlock::Match(ASTMatch {
                    matched: self.input.consume_until("}")?.to_string(),
                    attributes: self.collected_attributes.pop_vec(),
                    range: self.input.range_here(start),
                    choices: {
                        // Make sure the choice is not on the same line
                        self.input.skip_until_end_of_line();
                        self.parse_choice_list(depth, true, false)?.choices
                    },
                    kind
                }))
            },
            '-' => {
                Some(ASTBlock::ChoiceGroup(self.parse_choice_list(depth, false, true)?))
            },
            ' ' | '\n' => {
                self.input.skip();
                self.parse_block(depth)
            }
            _ => {
                let paragraph = self.parse_paragraph()?;
                if paragraph.tail.is_empty() && paragraph.parts.is_empty() {
                    None
                } else {
                    Some(ASTBlock::Paragraph(ASTParagraph {
                        parts: paragraph.parts,
                        tail: paragraph.tail,
                        range: paragraph.range,
                        attributes: self.collected_attributes.pop_vec()
                    }))
                }
            }
        }
    }

    /// "skip_depth_check" will only skip the depth check of the first line it encounters,
    /// this is because in some cases the identation may already be skipped
    pub fn parse_choice_list(&mut self, current_depth: u8, require_js: bool, skip_depth_check: bool) -> Option<ASTChoiceGroup> {
        let mut choices: Vec<ASTChoice> = vec![];
        let attributes = self.collected_attributes.pop_vec();
        let start = self.input.pos;
        while !self.input.is_eof() {
            if !skip_depth_check || !choices.is_empty() {
                let ident = self.input.get_identation();
                if current_depth != ident.0 {
                    break;
                }
                self.input.set_pos(ident.1);
            }
            match self.input.peek()? {
                '-' => {
                    self.input.skip();
                    if self.input.peek().is(' ') { self.input.skip() };
                    let attributes = if self.input.peek().is('#') && self.input.peek_n(1).is('[') {
                        self.input.skip_n(2);
                        self.parse_attributes()
                    } else { vec![] };
                    if self.input.peek().is(' ') { self.input.skip() };
                    let start = self.input.pos;
                    choices.push(ASTChoice {
                        text: {
                            let text = self.parse_paragraph();
                            let unwrapped = text?;
                            if require_js && (unwrapped.parts.is_empty() || !matches!(unwrapped.parts[0].text.kind, ASTInlineKind::Javascript(_))) {
                                self.input.ctx.add_diagnostic(dia!(REQUIRED_JS, self.input.range_here(start)));
                                continue;
                            } else { unwrapped }
                        },
                        children: self.parse_children(current_depth + 1),
                        attributes,
                        range: self.input.range_here(start),
                    })
                }
                _ => break,
            }
        }
        Some(ASTChoiceGroup {
            choices,
            range: self.input.range_here(start),
            attributes,
        })
    }

    pub fn parse_children(&mut self, depth: u8) -> Vec<ASTBlock> {
        let mut res = vec![];
        while !self.input.is_eof() {
            let ident = self.input.get_identation();
            if depth != ident.0 {
                break;
            }
            self.input.set_pos(ident.1);
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

    pub fn parse_string_list(&mut self, until: char) -> Vec<String> {
        let mut result = vec![];
        let mut current = String::new();
        let start = self.input.pos;
        while !self.input.is_eol() {
            let character = self.input.force_next();
            match character {
                ',' =>  {
                    if self.input.peek().is(' ') {
                        self.input.skip();
                    }
                    result.push(current.clone());
                    current.clear();
                },
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | ' ' | '-' | '>' | '#' | '@' | '$' | '*' | '^' | '.' => current.push(character),
                other if other == until => {
                    result.push(current.clone());
                    current.clear();
                    return result;
                },
                _ => {
                    self.input.back(1);
                    break;
                }
            }
        }
        // If this is executed it means the method didn't find anything
        self.input.ctx.add_diagnostic(dia!(MISSING_CLOSING, self.input.range_here(start), &until.to_string()));
        result
    }

    pub fn parse_attributes(&mut self) -> Vec<ASTAttribute> {
        let mut current_att = String::new();
        let mut last_start = self.input.pos;
        let mut parameters = vec![];
        let mut attributes = vec![];
        let start = self.input.pos;
        while !self.input.is_eol() {
            match self.input.force_next() {
                ']' => {
                    attributes.push(ASTAttribute {
                        name: current_att.clone(),
                        parameters: parameters.clone_and_empty(),
                        range: self.input.range_here(last_start)
                    });
                    current_att.clear();
                    return attributes;
                },
                ',' => {
                    if self.input.peek().is(' ') {
                        self.input.skip();
                    }
                    attributes.push(ASTAttribute {
                        name: current_att.clone(),
                        parameters: parameters.clone_and_empty(),
                        range: self.input.range_here(last_start)
                    });
                    current_att.clear();
                    last_start = self.input.pos;
                },
                '(' => parameters = self.parse_string_list(')'),
                other => current_att.push(other)
            }
        }
        // If this is executed it means the method didn't find anything
        self.input.ctx.add_diagnostic(dia!(MISSING_CLOSING, self.input.range_here(start), "]"));
        attributes
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
                                range: self.input.range_here(start)
                            },
                        });
                        result.clear()
                    },
                    '<' if self.input.peek().is('-') && self.input.peek_n(1).is('>') => {
                        self.input.skip_n(2);
                        if self.input.peek().is(' ') {
                            self.input.skip()
                        }
                        let text = self.parse_path_access();
                        parts.push(TextPart {
                            before: result.clone(),
                            text: ASTInline {
                                kind: ASTInlineKind::TempDivert(text),
                                range: self.input.range_here(start)
                            },
                        });
                        result.clear()
                    },
                    '{' => {
                        if let Some(text) = self.input.consume_until("}") {
                            parts.push(TextPart {
                                before: result.clone(),
                                text: ASTInline {
                                    kind: ASTInlineKind::Javascript(text.to_string()),
                                    range: self.input.range_here(start)
                                },
                            });
                            result.clear()
                        } else {
                            result.push('}')
                        }
                    },
                    other => result.push(other),
                }
            }
            InlineTextParseResult::FoundClosing(ASTText {
                parts,
                tail: result,
                range: self.input.range_here(start),
            })
        }
    }

    pub fn parse(mut self) -> (Vec<ASTBlock>, P) {
        let mut res = vec![];
        while !self.input.is_eof() {
            if let Some(block) = self.parse_block(0) {
                res.push(block);
            }
        }
        (res, self.input.ctx)
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
        let (input, ctx) = Parser::new(
            "
# This is a chapter

This is a **paragraph**...

@{:if match_condition}
- {true}
    ```js
    This is a language!
    ```
    So yeah, what's up?
    Wohoooo!
    @{nested_match}
    - {a == 1}
        This is a child!
    - {}
        Another match...
        @{double_nested_match}
        - {REALLY_NESTED!!}
            Wow!!!
        - {Really nestes 2!!}
        - {Really nested 3!!!}
- {false}
    The option is false!
- Third option...
",
            Context::new(),
        )
        .parse();
        assert_eq!(ctx.errors.len(), 1);
        if let ASTBlock::Match(matcher) = &input[2] {
            assert!(matches!(matcher.kind, MatchKind::If));
            assert_eq!(matcher.matched, "match_condition");
            // 2 because "Third option..." doesn't get included because JS is required
            assert_eq!(matcher.choices.len(), 2);
            assert_eq!(matcher.choices[0].text.to_raw(), "true");
            assert_eq!(matcher.choices[1].text.to_raw(), "false");
            if let ASTBlock::Match(nested_match) = &matcher.choices[0].children[3] {
                assert_eq!(nested_match.choices.len(), 2);
                assert_eq!(nested_match.choices[0].text.to_raw(), "a == 1");
                assert_eq!(nested_match.choices[1].text.to_raw(), "");
                if let ASTBlock::Match(triple_nested) = &nested_match.choices[1].children[1] {
                    assert_eq!(triple_nested.choices.len(), 3);
                } else {
                    panic!("Triple nested")
                }
            } else {
                panic!("Nested match")
            }
        } else {
            panic!("Match")
        }
    }

    #[test]
    fn parse_attributes() {
        let (input, ctx) = Parser::new("
# Hello World!

#[Uppercase(A, Bcccc, C), DebugTitle(Some choice...)]
This is a paragraph with an attribute in it!

#[SomeThing(123]
Another paragraph...
        ", Context::new()).parse();
        assert_eq!(ctx.errors.len(), 1);
        if let ASTBlock::Paragraph(para) = &input[1] {
            println!("{:?}", para.attributes);
            assert_eq!(para.attributes.len(), 2);
            assert_eq!(para.attributes[0].name, "Uppercase");
            assert_eq!(para.attributes[0].parameters[0], "A");
            assert_eq!(para.attributes[0].parameters[1], "Bcccc");
            assert_eq!(para.attributes[0].parameters[2], "C");
            assert_eq!(para.attributes[1].name, "DebugTitle");
            assert_eq!(para.attributes[1].parameters[0], "Some choice...");
        } else {
            panic!("Paragraph")
        }
    }

    #[test]
    fn parse_choice_group() {
        let (input, _) = Parser::new("
# Hello World!

It's time to choose...

#[ChoiceGroupAttribute]
- Option A, ooor..
- Option B...
    So you chose option B, **now* it's time...
    - Option C
        You chose option C
    - #[SomeAttribute] Option D
        You chose option D
        ", Context::new()).parse();
        println!("{:?}", input);
        if let ASTBlock::ChoiceGroup(para) = &input[2] {
            assert_eq!(para.attributes[0].name, "ChoiceGroupAttribute");
            assert_eq!(para.choices[0].text.to_raw(), "Option A, ooor..");
            assert_eq!(para.choices[1].text.to_raw(), "Option B...");
            assert_eq!(para.choices[1].children.len(), 2);
            if let ASTBlock::ChoiceGroup(nested) = &para.choices[1].children[1] {
                assert_eq!(nested.choices[0].text.to_raw(), "Option C");
                assert_eq!(nested.choices[0].children.len(), 1);
                assert_eq!(nested.choices[1].text.to_raw(), "Option D");
                assert_eq!(nested.choices[1].children.len(), 1);
                assert_eq!(nested.choices[1].attributes[0].name, "SomeAttribute");
            }
        } else {
            panic!("Choice Group")
        }
    }
}
