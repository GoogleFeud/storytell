use storytell_diagnostics::diagnostic::{StorytellResult, Diagnostic};
use storytell_diagnostics::location::Range;
use storytell_js_parser::JsParser;
use storytell_parser::ast::model::*;
use std::{stringify, concat};

use crate::base::files::BlobId;
use crate::json_compiler::JSONCompilerContext;
use crate::path::Path;
use crate::visitors::{VariableCollector, Rebuilder, transform_js};

#[macro_export]
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
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String>;
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
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        let mut header_children: Vec<String> = vec![];
        let mut others: Vec<&ASTBlock> = vec![];
        for child in &self.children {
            if let ASTBlock::Header(header) = &child {
                header_children.push(format!("\"{}\": {}", Path::canonicalize_name(&header.title.text), header.compile(ctx, file_id)?));
            } else {
                others.push(child)
            }
        }
        Ok(json!({
            title: self.title.text.safe_compile(),
            canonicalTitle: Path::canonicalize_name(&self.title.text).safe_compile(),
            childPaths: format!("{{{}}}", header_children.join(",")),
            range: self.range.safe_compile(),
            children: others.compile(ctx, file_id)?
        }))
    }
}

impl JSONCompilable for ASTInline {
    /// `Inline` type
    /// {
    ///   "kind": InlineKind,
    ///   "text"?: Text,
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
    /// Join - 4
    /// Javascript - 5
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        Ok(match &self.kind {
            ASTInlineKind::Bold(text) => json!({
                kind: 0,
                text: text.compile(ctx, file_id)?,
                range: text.range.safe_compile()
            }),
            ASTInlineKind::Italics(text) => json!({
                kind: 1,
                text: text.compile(ctx, file_id)?,
                range: text.range.safe_compile()
            }),
            ASTInlineKind::Underline(text) => json!({
                kind: 2,
                text: text.compile(ctx, file_id)?,
                range: text.range.safe_compile()
            }),
            ASTInlineKind::Code(text) =>json!({
                kind: 3,
                text: text.compile(ctx, file_id)?,
                range: text.range.safe_compile()
            }),
            ASTInlineKind::Join => json!({
                kind: 4,
                range: self.range.safe_compile()
            }),
            ASTInlineKind::Javascript(text) => {
                let (expressions, diagnostics, input) = JsParser::parse(text);
                if !diagnostics.is_empty() {
                    return Err(diagnostics.into_iter().map(|d| Diagnostic {
                        msg: d.msg,
                        variant: d.variant,
                        range: Range::new(self.range.start + d.range.start + 1, self.range.start + d.range.end - 1)
                    }).collect::<Vec<Diagnostic>>())
                } else {
                    let input = VariableCollector {
                        input,
                        current_origin: file_id,
                        store: &mut ctx.variables,
                        start_pos: Range::new(self.range.start + 1, self.range.end - 1)
                    }.run(&expressions);
                    let rebuilt_code = Rebuilder::run(input, &expressions, ctx.prefix_js_idents.clone());
                    json!({
                        kind: 5,
                        text: format!("\"{}\"", rebuilt_code),
                        range: self.range.safe_compile()
                    })
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
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        Ok(json!({
            parts: self.parts.compile(ctx, file_id)?,
            tail: self.tail.safe_compile(),
            range: self.range.safe_compile()
        }))
    }
}

impl JSONCompilable for ASTParagraph {
    /// `Paragraph` type
    /// {
    ///     kind: 0,
    ///     parts: TextPart[],
    ///     tail: string
    /// }
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        Ok(json!({
            kind: 0,
            parts: self.parts.compile(ctx, file_id)?,
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
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        Ok(json!({
            before: self.before.safe_compile(),
            text: self.text.compile(ctx, file_id)?
        }))
    }
}

impl JSONCompilable for ASTCodeBlock {
    /// `CodeBlock` type
    /// {
    ///     kind: 1,
    ///     code: string,
    ///     language: string
    /// }
    fn compile(&self, _ctx: &mut JSONCompilerContext, _file_id: BlobId) -> StorytellResult<String> {
        Ok(json!({
            kind: 1,
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
    ///     children: Block[],
    ///     condition?: {
    ///         modifier: string,
    ///         text: string
    ///     }
    /// }
    /// 
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        Ok(json!({
            text: self.text.compile(ctx, file_id)?,
            children: self.children.compile(ctx, file_id)?,
            range: self.range.safe_compile(),
            attributes: self.attributes.safe_compile(),
            condition: self.condition.as_ref().map(|c| json!({
                modifier: c.0.safe_compile(),
                text: c.1.safe_compile()
            })).safe_compile()
        }))
    }
}

impl JSONCompilable for ASTChoiceGroup {
    /// `ChoiceGroup` type
    /// {
    ///     kind: 2,
    ///     choices: Choice[]
    /// }
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        Ok(json!({
            kind: 2,
            choices: self.choices.compile(ctx, file_id)?,
            range: self.range.safe_compile(),
            attributes: self.attributes.safe_compile()
        }))
    }
}

impl JSONCompilable for ASTDivert {
    /// `Divert` type
    /// {
    ///     kind: 3,
    ///     path: string[]
    /// }
    /// 
    fn compile(&self, _ctx: &mut JSONCompilerContext, _file_id: BlobId) -> StorytellResult<String> {
        Ok(json!({
            kind: 3,
            path: self.path.safe_compile(),
            range: self.range.safe_compile(),
            attributes: self.attributes.safe_compile()
        }))
    }
}

impl JSONCompilable for ASTMatch {
    /// `Match` type
    /// {
    ///     kind: 4,
    ///     condition: string,
    ///     modifier?: string,
    ///     arms: Choice[],
    ///     children: Block[]
    /// }
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        let mut choices: Vec<String> = vec![];
        for choice in &self.choices {
            choices.push(json!({ 
                text: format!("\"{}\"", transform_js(&choice.text.parts[0].text.to_raw(), ctx.prefix_js_idents.clone())?),
                children: choice.children.compile(ctx, file_id)?,
                range: choice.range.safe_compile(),
                attributes: choice.attributes.safe_compile()
            }));
        }
        Ok(json!({
            kind: 4,
            condition: format!("\"{}\"", transform_js(&self.matched, ctx.prefix_js_idents.clone())?),
            modifier: self.kind.safe_compile(),
            arms: format!("[{}]", choices.join(","))
        }))
    }
}

impl JSONCompilable for ASTBlock {
    /// `Block` enum type
    /// 
    /// `BlockKind` enum
    /// Paragraph - 0
    /// CodeBlock - 1
    /// ChoiceGroup - 2
    /// Divert - 3
    /// Match - 4
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        match self {
            Self::Header(header) => header.compile(ctx, file_id),
            Self::Paragraph(paragraph) => paragraph.compile(ctx, file_id),
            Self::CodeBlock(code) => code.compile(ctx, file_id),
            Self::ChoiceGroup(group) => group.compile(ctx, file_id),
            Self::Divert(divert) => divert.compile(ctx, file_id),
            Self::Match(match_exp) => match_exp.compile(ctx, file_id)
        }
    }
}

impl JSONCompilable for &ASTBlock {
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        match self {
            ASTBlock::Header(header) => header.compile(ctx, file_id),
            ASTBlock::Paragraph(paragraph) => paragraph.compile(ctx, file_id),
            ASTBlock::CodeBlock(code) => code.compile(ctx, file_id),
            ASTBlock::ChoiceGroup(group) => group.compile(ctx, file_id),
            ASTBlock::Divert(divert) => divert.compile(ctx, file_id),
            ASTBlock::Match(match_exp) => match_exp.compile(ctx, file_id)
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
        format!("\"{}\"", self.replace('\n', "\\n").replace('\r', "\\r").replace('"', "\\\""))
    }
}

impl<T: JSONSafeCompilable> JSONSafeCompilable for Option<T> {
    fn safe_compile(&self) -> String {
        if let Some(value) = self {
            value.safe_compile()
        } else {
            String::from("null")
        }
    }
}

impl<T: JSONCompilable> JSONCompilable for Vec<T> {
    fn compile(&self, ctx: &mut JSONCompilerContext, file_id: BlobId) -> StorytellResult<String> {
        Ok(format!("[{}]", self.iter().map(|i| i.compile(ctx, file_id)).collect::<StorytellResult<Vec<String>>>()?.join(",")))
    }
}

impl<T: JSONSafeCompilable> JSONSafeCompilable for Vec<T> {
    fn safe_compile(&self) -> String {
        format!("[{}]", self.iter().map(|i| i.safe_compile()).collect::<Vec<String>>().join(","))
    }
}