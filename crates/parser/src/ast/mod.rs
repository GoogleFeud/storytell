pub mod model;
pub mod utils;

use self::utils::*;
use crate::input::*;
use model::*;
use storytell_diagnostics::{diagnostic::*, make_diagnostics, dia};

make_diagnostics!(define [
    REQUIRED_JS,
    P1001,
    "Match condition must be a javascript inline expression."
], [
    MISSING_CLOSING,
    P1002,
    "Missing closing character '$'."
], [
    NESTED_HEADER,
    P1003,
    "Path start cannot be inside options."
], [
    INCORRECT_HEADER_SIZE,
    P1004,
    "Path should be one ($) level deeper than it's parent."
], [
    NO_CONDITION,
    P1005,
    "Match options cannot have conditions."
]);

pub enum InlineTextParseResult {
    FoundClosing(ASTText),
    NotFoundOptional(ASTText),
    NotFound,
}

pub struct Parser<'a> {
    input: InputConsumer<'a>,
    collected_attributes: VecStack<ASTAttribute>,
    header_counter: u32
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str, ctx: ParsingContext) -> Self {
        Self {
            input: InputConsumer::new(text, ctx),
            collected_attributes: VecStack::new(),
            header_counter: 0
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
                if depth != 0 {
                    self.input.ctx.diagnostics.push(dia!(NESTED_HEADER, self.input.range_here(start)));
                }
                let header_depth = 1 + self.input.count_while('#');
                let header_id = self.header_counter;
                self.header_counter += 1;
                let header_text = self.input.consume_until_end_of_line().trim().to_string();
                Some(ASTBlock::Header(ASTHeader {
                    canonical_title: ASTHeader::canonicalize_name(&header_text),
                    title: ASTPlainText {
                        text: header_text,
                        range: self.input.range_here(start)
                    },
                    depth: header_depth as u8,
                    attributes: self.collected_attributes.pop_vec(),
                    children: {
                        let mut res = vec![];
                        loop {
                            let hash_count = self.input.count('#');
                            if hash_count > 0 && hash_count <= header_depth {
                                break;
                            }
                            else if let Some(block) = self.parse_block(depth) {
                                if let ASTBlock::Header(header) = &block {
                                    if (header.depth - 1) > (header_depth as u8) {
                                        self.input.ctx.diagnostics.push(dia!(INCORRECT_HEADER_SIZE, header.title.range.clone(), &(header.depth - 1).to_string()));
                                    }
                                }
                                res.push(block);
                            } else {
                                break;
                            }
                        }
                        res
                    },
                    range: self.input.range_here(start),
                    id: header_id
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
                    self.input.consume_until(" ").map(|v| v.to_string())
                } else { None };
                Some(ASTBlock::Match(ASTMatch {
                    matched: self.input.consume_until("}")?.to_string(),
                    attributes: self.collected_attributes.pop_vec(),
                    range: self.input.range_here(start),
                    direct_children: if kind.is_some() {
                        self.input.skip_until_end_of_line();
                        self.parse_children(depth + 1)
                    } else { vec![] },
                    choices: if kind.is_none() {
                        self.input.skip_until_end_of_line();
                        self.parse_choice_list(depth, true, false)?.choices
                    } else { vec![] },
                    kind
                }))
            },
            '-' if self.input.peek_n(1).is('>') => {
                self.input.skip_n(2);
                    if self.input.peek().is(' ') {
                        self.input.skip();
                    }
                    let path = self.parse_path_access();
                    Some(ASTBlock::Divert(ASTDivert {
                        path,
                        range: {
                            let range = self.input.range_here(start);
                            self.input.consume_until_end_of_line();
                            range
                        },
                        attributes: self.collected_attributes.pop_vec()
                    }))
            },
            '-' => {
                Some(ASTBlock::ChoiceGroup(self.parse_choice_list(depth, false, true)?))
            },
            '/' if self.input.peek_n(1).is('/') => {
                self.input.skip_until_end_of_line();
                self.parse_block(depth)
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
                    let condition = if self.input.peek().is('{') && self.input.peek_n(1).is(':') {
                        if require_js {
                            self.input.ctx.diagnostics.push(dia!(NO_CONDITION, self.input.range_single()));
                        }
                        self.input.skip_n(2);
                        let kind = self.input.consume_until(" ")?.to_string();
                        Some((kind, self.input.consume_until("}")?.to_string()))
                    } else { None };
                    if self.input.peek().is(' ') { self.input.skip() };
                    let start = self.input.pos;
                    choices.push(ASTChoice {
                        text: {
                            let text = self.parse_paragraph();
                            let unwrapped = text?;
                            if require_js && (unwrapped.parts.is_empty() || !matches!(unwrapped.parts[0].text.kind, ASTInlineKind::Javascript(_))) {
                                self.input.ctx.diagnostics.push(dia!(REQUIRED_JS, self.input.range_here(start)));
                                continue;
                            } else { unwrapped }
                        },
                        children: self.parse_children(current_depth + 1),
                        attributes,
                        condition,
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
        match self.parse_text(resolve_line_endings(self.input.ctx.line_endings), true) {
            Some(text) => {
                self.input.skip_n(self.input.ctx.line_endings);
                Some(text)
            }
            None => None
        }
    }

    pub fn parse_path_access(&mut self) -> Vec<String> {
        let mut paths: Vec<String> = vec![];
        let mut current_path = String::new();
        while !self.input.is_eol() {
            match self.input.force_next() {
                ch @ '0'..='9' | ch @ 'a'..='z' | ch @ '_' => {
                    current_path.push(ch)
                },
                ch @ 'A'..='Z' => {
                    current_path.push(ch.to_lowercase().next().unwrap())
                }
                '.' => {
                    paths.push(current_path.clone());
                    current_path.clear()
                }
                _ => {
                    self.input.back(1);
                    break;
                },
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
        self.input.ctx.diagnostics.push(dia!(MISSING_CLOSING, self.input.range_here(start), &until.to_string()));
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
        self.input.ctx.diagnostics.push(dia!(MISSING_CLOSING, self.input.range_here(start), "]"));
        attributes
    }

    pub fn parse_text(&mut self, until: &str, optional: bool) -> Option<ASTText> {
        let mut parts: Vec<TextPart> = vec![];
        let mut result = String::new();
        let start = self.input.pos;
        let pos_end = self.input.get_pos_of(until).unwrap_or({
            if optional {
                self.input.data.len()
            } else {
                0
            }
        });
        if pos_end == 0 {
            None
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
                        if let Some(text) = self.parse_text("**", false) {
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
                    '+' if self.input.peek().is('+') => {
                        self.input.skip();
                        parts.push(TextPart { 
                            before: result.clone(), 
                            text: ASTInline {
                                kind: ASTInlineKind::Join,
                                range: self.input.range_here(start)
                            }
                        });
                        result.clear()
                    }
                    // Italics
                    '*' => {
                        let start = self.input.pos - 1;
                        if let Some(text) = self.parse_text("*", false) {
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
                    // Code
                    '`' => {
                        let start = self.input.pos - 1;
                        if let Some(text) = self.parse_text("`", false) {
                            self.input.skip();
                            parts.push(TextPart {
                                before: result.clone(),
                                text: ASTInline {
                                    kind: ASTInlineKind::Code(text),
                                    range: self.input.range_here(start),
                                },
                            });
                            result.clear()
                        } else {
                            result.push('`');
                        }
                    },
                    '{' => {
                        if let Some(text) = self.input.consume_until_of_eol("}") {
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
            Some(ASTText {
                parts,
                tail: result,
                range: self.input.range_here(start),
            })
        }
    }

    pub fn parse(mut self) -> (Vec<ASTBlock>, ParsingContext) {
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
    use super::*;

    fn get_header_children(input: &Vec<ASTBlock>) -> &Vec<ASTBlock> {
        if let ASTBlock::Header(header) = &input[0] {
            &header.children
        } else {
            panic!("Expected a header.")
        }
    }

    #[test]
    fn parse_header() {
        let (input, ctx) = Parser::new("
# This is some header!!!...
This is a paragraph, a child of the header...
## This is a second header, child of the first header
Blah Blah Blah
### This is a third header, child of the second
Text...
#### Fourth header
###### Wrong header!!!
## This is child of first
        ", ParsingContext::new(1)).parse();
        assert_eq!(ctx.diagnostics[0].msg, "Path should be one (5) level deeper than it's parent.");
        let header = &input[0];
        if let ASTBlock::Header(block) = header {
            assert_eq!(block.title.text, "This is some header!!!...");
            assert_eq!(block.depth, 1);
            assert_eq!(block.children.len(), 3);
            if let ASTBlock::Header(header) = &block.children[1] {
                assert_eq!(header.children.len(), 2);
                if let ASTBlock::Header(header) = &header.children[1] {
                    assert_eq!(header.children.len(), 2);
                    if let ASTBlock::Header(header) = &header.children[1] {
                        assert_eq!(header.children.len(), 1);
                    } else {
                        panic!("Expected header")
                    }
                } else {
                    panic!("Expected header")
                }
            } else {
                panic!("Expected header")
            }
        } else {
            panic!("Expected header")
        }
    }

    #[test]
    fn parse_codeblock() {
        let (input, _) = Parser::new(
            "# This is some header!!!...\n```js\nThis is a code\nblock...\nyeah...```",
            ParsingContext::new(1),
        ).parse();
        let header = get_header_children(&input);
        let code_block = &header[0];
        if let ASTBlock::CodeBlock(block) = code_block {
            assert_eq!(block.text, "This is a code\nblock...\nyeah...");
            assert_eq!(block.language, "js");
        } else {
            panic!("Code block")
        }
    }

    #[test]
    fn parse_inline_bold() {
        let (input, _) = Parser::new("# This is some header!!!...\nThis is **a** paragraph, pretty cool... **really** cool! Same paragraph...\nAlright this is a different one!!## Another heading", ParsingContext::new(1)).parse();
        let children = get_header_children(&input);
        if let ASTBlock::Paragraph(para) = &children[0] {
            assert_eq!(
                "This is a paragraph, pretty cool... really cool! Same paragraph...",
                para.to_raw()
            );
        } else {
            panic!("Expected paragraph")
        }
    }

    #[test]
    fn parse_inline_italics() {
        let (input, _) = Parser::new(
            "# This is some header!!!...\n**really** interesting *word*...\nAlright",
            ParsingContext::new(1),
        ).parse();
        let children = get_header_children(&input);
        if let ASTBlock::Paragraph(para) = &children[0] {
            assert_eq!(
                "really interesting word...",
                para.to_raw()
            );
            assert!(matches!(
                para.parts[1].text.kind,
                ASTInlineKind::Italics(_)
            ));
        } else {
            panic!("Expected paragraph")
        }
    }

    #[test]
    fn parse_choice_list() {
        let (input, ctx) = Parser::new(
            "
# This is a chapter

This is a **paragraph**...

@{match_condition}
- {:if condition} {true}
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
- {fourth}
    @{:not killed}
        Direct child!
        Second direct child...
        - Some choices
        - Cause why not...
",
ParsingContext::new(1),
        )
        .parse();
        let children = get_header_children(&input);
        println!("{:?}", ctx.diagnostics);
        assert_eq!(ctx.diagnostics.len(), 2);
        if let ASTBlock::Match(matcher) = &children[1] {
            assert_eq!(matcher.matched, "match_condition");
            // 3 because "Third option..." doesn't get included because JS is required
            assert_eq!(matcher.choices.len(), 3);
            assert_eq!(matcher.choices[0].text.to_raw(), "true");
            assert_eq!(matcher.choices[1].text.to_raw(), "false");
            if let ASTBlock::Match(not_match) = &matcher.choices[2].children[0] {
                assert_eq!(not_match.kind, Some("not".to_string()));
                assert_eq!(not_match.direct_children.len(), 3);
                assert_eq!(not_match.choices.len(), 0);
            } else {
                panic!("Not match")
            }
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
        ", ParsingContext::new(1)).parse();
        let children = get_header_children(&input);
        assert_eq!(ctx.diagnostics.len(), 1);
        if let ASTBlock::Paragraph(para) = &children[0] {
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
    - {:if a == b} Option C
        You chose option C
    - #[SomeAttribute] Option D
        You chose option D
        ", ParsingContext::new(1)).parse();
        println!("{:?}", input);
        let children = get_header_children(&input);
        if let ASTBlock::ChoiceGroup(para) = &children[1] {
            assert_eq!(para.attributes[0].name, "ChoiceGroupAttribute");
            assert_eq!(para.choices[0].text.to_raw(), "Option A, ooor..");
            assert_eq!(para.choices[1].text.to_raw(), "Option B...");
            assert_eq!(para.choices[1].children.len(), 2);
            if let ASTBlock::ChoiceGroup(nested) = &para.choices[1].children[1] {
                assert_eq!(nested.choices[0].text.to_raw(), "Option C");
                assert_eq!(nested.choices[0].condition, Some(("if".to_string(), "a == b".to_string())));
                assert_eq!(nested.choices[0].children.len(), 1);
                assert_eq!(nested.choices[1].text.to_raw(), "Option D");
                assert_eq!(nested.choices[1].children.len(), 1);
                assert_eq!(nested.choices[1].attributes[0].name, "SomeAttribute");
            }
        } else {
            panic!("Choice Group")
        }
    }

    #[test]
    fn parse_comment() {
        let (input, _) = Parser::new("
# Hello World!
// A comment
// # A second comment...
## A sub-path", ParsingContext::new(1)).parse();
        let children = get_header_children(&input);
        assert!(matches!(input[0], ASTBlock::Header(_)));
        assert!(matches!(children[0], ASTBlock::Header(_)));
    }
}
