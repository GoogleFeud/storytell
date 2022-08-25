use storytell_diagnostics::diagnostic::StorytellResult;
use storytell_diagnostics::location::Range;
use storytell_js_parser::JsParser;
use storytell_js_parser::ast::Visitable;
use storytell_parser::ast::model::*;
use std::{stringify, concat};

use crate::json_compiler::JSONCompilerContext;
use crate::path::Path;
use crate::visitors::{MagicVarCollector, Rebuilder};


macro_rules! json {
    () => {};
    ($key: ident:$value: expr, $($tail:tt)*) => {
        concat!("\"", stringify!($key), "\":{},", json!($($tail)*))
    };
    ($key: ident:$value: expr) => {
        concat!("\"", stringify!($key), "\":{}")
    };
    ({$($property: ident: $value: expr),+}) => {
        format!(concat!("{{", json!($($property: $value),+), "}}"), $($value),+)
    };
}

pub trait JSONCompilable {
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String>;
}

pub trait JSONSafeCompilable {
    fn safe_compile(&self) -> String;
}

impl JSONCompilable for ASTHeader {
    /// `Path` object:
    /// {
    ///  "title": string,
    ///  "canonicalTitle": string,
    ///  "childPaths": Path[],
    ///  "children": []
    /// }
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        let mut header_children: Vec<String> = vec![];
        let mut others: Vec<&ASTBlock> = vec![];
        for child in &self.children {
            if let ASTBlock::Header(header) = &child {
                header_children.push(format!("\"{}\": {}", Path::canonicalize_name(&header.title.text), header.compile(ctx)?));
            } else {
                others.push(child)
            }
        }
        Ok(json!({
            title: self.title.text.safe_compile(),
            canonicalTitle: Path::canonicalize_name(&self.title.text).safe_compile(),
            childPaths: format!("{{{}}}", header_children.join(",")),
            range: self.range.safe_compile(),
            children: others.compile(ctx)?
        }))
    }
}

impl JSONCompilable for ASTInline {
    /// `Inline` type
    /// {
    ///   "kind": InlineKind,
    ///   "text": Text,
    ///   "magicVariables"?: {
    ///     "name": string,
    ///     "kind": number
    ///   }[]
    /// }
    /// 
    /// `InlineKind` enum:
    /// Bold - 0
    /// Italics - 1
    /// Underline - 2
    /// Code - 3
    /// Javascript - 4
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        Ok(match &self.kind {
            ASTInlineKind::Bold(text) => json!({
                kind: 0,
                text: text.compile(ctx)?,
                range: text.range.safe_compile()
            }),
            ASTInlineKind::Italics(text) => json!({
                kind: 1,
                text: text.compile(ctx)?,
                range: text.range.safe_compile()
            }),
            ASTInlineKind::Underline(text) => json!({
                kind: 2,
                text: text.compile(ctx)?,
                range: text.range.safe_compile()
            }),
            ASTInlineKind::Code(text) =>json!({
                kind: 3,
                text: text.compile(ctx)?,
                range: text.range.safe_compile()
            }),
            ASTInlineKind::Javascript(text) => {
                let (expressions, diagnostics, input) = JsParser::parse(text);
                if !diagnostics.is_empty() {
                    return Err(diagnostics)
                } else {
                    let mut magic_vars_collector = MagicVarCollector::new(input, Range::new(self.range.start + 1, self.range.end - 1), &mut ctx.magic_variables);
                    expressions.visit_each_child(&mut magic_vars_collector);
                    if !magic_vars_collector.diagnostics.is_empty() {
                        return Err(magic_vars_collector.diagnostics)
                    } else {
                        let gathered_variables = magic_vars_collector.collected.iter().map(|pair| json!({ name: pair.0.safe_compile(), kind: pair.1 })).collect::<Vec<String>>();
                        let rebuilt_code = Rebuilder::run(magic_vars_collector.input, &expressions);
                        json!({
                            kind: 4,
                            text: format!("\"{}\"", rebuilt_code),
                            magicVariables: format!("[{}]", gathered_variables.join(",")),
                            range: self.range.safe_compile()
                        })
                    }
                }
            }
        })
    }
}

impl JSONCompilable for ASTText {
    /// `Text` type
    /// {
    ///     parts: TextPart[],
    ///     tail: string
    /// }
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        Ok(json!({
            parts: self.parts.compile(ctx)?,
            tail: self.tail.safe_compile(),
            range: self.range.safe_compile()
        }))
    }
}

impl JSONCompilable for ASTParagraph {
    /// `Paragraph` type
    /// {
    ///     parts: TextPart[],
    ///     tail: string
    /// }
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        Ok(json!({
            parts: self.parts.compile(ctx)?,
            tail: self.tail.safe_compile(),
            range: self.range.safe_compile(),
            attributes: self.attributes.safe_compile()
        }))
    }
}

impl JSONCompilable for TextPart {
    /// `TextPart` type
    /// {
    ///     before: string,
    ///     text: Inline
    /// }
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        Ok(json!({
            before: self.before.safe_compile(),
            text: self.text.compile(ctx)?
        }))
    }
}

impl JSONCompilable for ASTCodeBlock {
    /// `CodeBlock` type
    /// {
    ///     code: string,
    ///     language: string
    /// }
    fn compile(&self, _ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        Ok(json!({
            code: self.text.safe_compile(),
            language: self.language.safe_compile(),
            range: self.range.safe_compile(),
            attributes: self.attributes.safe_compile()
        }))
    }
}

impl JSONCompilable for ASTChoice {
    /// `Choice` type
    /// {
    ///     text: Text,
    ///     children: Block[]
    /// }
    /// 
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        Ok(json!({
            text: self.text.compile(ctx)?,
            children: self.children.compile(ctx)?,
            range: self.range.safe_compile(),
            attributes: self.attributes.safe_compile()
        }))
    }
}

impl JSONCompilable for ASTChoiceGroup {
    /// `ChoiceGroup` type
    /// {
    ///     choices: Choice[]
    /// }
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        Ok(json!({
            text: self.choices.compile(ctx)?,
            range: self.range.safe_compile(),
            attributes: self.attributes.safe_compile()
        }))
    }
}

impl JSONCompilable for ASTBlock {
    /// `Block` enum type
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        match self {
            Self::Header(header) => header.compile(ctx),
            Self::Paragraph(paragraph) => paragraph.compile(ctx),
            Self::CodeBlock(code) => code.compile(ctx),
            Self::ChoiceGroup(group) => group.compile(ctx),
            _ => Ok(String::from("\"\""))
        }
    }
}

impl JSONCompilable for &ASTBlock {
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        match self {
            ASTBlock::Header(header) => header.compile(ctx),
            ASTBlock::Paragraph(paragraph) => paragraph.compile(ctx),
            ASTBlock::CodeBlock(code) => code.compile(ctx),
            ASTBlock::ChoiceGroup(group) => group.compile(ctx),
            _ => Ok(String::from("\"\""))
        }
    }
}

impl JSONSafeCompilable for ASTAttribute {
    fn safe_compile(&self) -> String {
        json!({
            name: self.name.safe_compile(),
            parameters: self.parameters.safe_compile(),
            range: self.range.safe_compile()
        })
    }
}

impl JSONSafeCompilable for Range<usize> {
    fn safe_compile(&self) -> String {
        json!({
            start: self.start,
            end: self.end
        })
    }
}

impl JSONSafeCompilable for String {
    fn safe_compile(&self) -> String {
        format!("\"{}\"", self.replace('"', "\\\"").replace('\n', "\\\\n"))
    }
}

impl<T: JSONCompilable> JSONCompilable for Vec<T> {
    fn compile(&self, ctx: &mut JSONCompilerContext) -> StorytellResult<String> {
        Ok(format!("[{}]", self.iter().map(|i| i.compile(ctx)).collect::<StorytellResult<Vec<String>>>()?.join(",")))
    }
}

impl<T: JSONSafeCompilable> JSONSafeCompilable for Vec<T> {
    fn safe_compile(&self) -> String {
        format!("[{}]", self.iter().map(|i| i.safe_compile()).collect::<Vec<String>>().join(","))
    }
}