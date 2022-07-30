use super::{CompilerContext, Path};
use storytell_parser::ast::model::*;
use storytell_diagnostics::diagnostic::*;
use storytell_diagnostics::{dia, make_diagnostics};

make_diagnostics!(define [
    UNKNOWN_CHILD_PATH,
    2001,
    "\"$\" is not a sub-path of \"$\"."
], [
    UNKNOWN_PATH,
    2002,
    "\"$\" is not a path."
]);

pub trait JSCompilable {
    fn compile(&self, ctx: &mut CompilerContext) -> StroytellResult<String>;
}

pub trait JSSafeCompilable {
    fn safe_compile(&self) -> String { String::new() }
}

impl JSCompilable for ASTInline {
    fn compile(&self, ctx: &mut CompilerContext) -> StroytellResult<String> {
        match &self.kind {
            ASTInlineKind::Bold(text) => Ok(format!("<b>{}</b>", text.compile(ctx)?)),
            ASTInlineKind::Italics(text) => Ok(format!("<i>{}</i>", text.compile(ctx)?)),
            ASTInlineKind::Underline(text) => Ok(format!("<u>{}</u>", text.compile(ctx)?)),
            ASTInlineKind::Code(text) => Ok(format!("<code>{}</code>", text.compile(ctx)?)),
            ASTInlineKind::Javascript(text) => Ok(format!("${{{}}}", text)),
            ASTInlineKind::Divert(thing, is_temp) => {
                match ctx.paths[0].try_get_child_by_path(thing) {
                    Ok(_) => {
                        Ok(format!("${{{}([{}])}}", if *is_temp { ctx.bootstrap.temp_divert_fn } else { ctx.bootstrap.divert_fn }, thing.iter().map(|string| format!("\"{}\"", string)).collect::<Vec<String>>().join(", ")))
                    },
                    Err(ind) => {
                        if ind == 0 {
                            Err(dia!(UNKNOWN_PATH, self.range.clone(), &thing[ind]))
                        } else {
                            Err(dia!(UNKNOWN_CHILD_PATH, self.range.clone(), &thing[ind], &thing[ind - 1]))
                        }
                    }
                }
            }
        }
    }
}

impl JSCompilable for ASTText {
    fn compile(&self, ctx: &mut CompilerContext) -> StroytellResult<String> {
        if self.parts.is_empty() {
            return Ok(self.tail.clone())
        }
        let mut result = String::new();
        for part in &self.parts {
            result.push_str(&part.before);
            result.push_str(&part.text.compile(ctx)?)
        }
        result.push_str(&self.tail);
        Ok(result)
    }
}

impl JSCompilable for ASTHeader {
    /// Transpiles to an object that looks like this:
    /// ```js
    /// {
    ///     title: string,
    ///     canonicalTitle: string,
    ///     childPaths: [self],
    ///     children: []
    /// }
    fn compile(&self, ctx: &mut CompilerContext) -> StroytellResult<String> {
        let mut header_children: Vec<String> = vec![];
        let mut others: Vec<&ASTBlock> = vec![];
        for child in &self.children {
            if let ASTBlock::Header(header) = &child {
                header_children.push(format!("{}: {}", Path::canonicalize_name(&header.title.text), header.compile(ctx)?));
            } else {
                others.push(child)
            }
        }
        Ok(format!("{{
            title: {},
            canonicalTitle: {},
            childPaths: {{{}}}
        }}", 
        self.title.text.safe_compile(), 
        Path::canonicalize_name(&self.title.text).safe_compile(),
        header_children.join(",")
        ))
    }
}

impl JSCompilable for ASTParagraph {
    fn compile(&self, ctx: &mut CompilerContext) -> StroytellResult<String> {
        if self.parts.is_empty() {
            return Ok(self.tail.clone())
        }
        let mut result = String::new();
        for part in &self.parts {
            result.push_str(&part.before);
            result.push_str(&part.text.compile(ctx)?)
        }
        result.push_str(&self.tail);
        let para = ctx.bootstrap.paragraph_fn;
        Ok(format!("{}({}, {})", para, result.safe_compile(), self.attributes.compile(ctx)?))
    }
}

impl JSCompilable for ASTCodeBlock {
    fn compile(&self, ctx: &mut CompilerContext) -> StroytellResult<String> {
        let codeblock_fn = ctx.bootstrap.codeblock_fn;
        Ok(format!("{}({}, {}, {})", codeblock_fn, self.text.safe_compile(), self.language.safe_compile(), self.attributes.compile(ctx)?))
    }
}

impl JSCompilable for ASTMatch {
    fn compile(&self, ctx: &mut CompilerContext) -> StroytellResult<String> {
        Ok(format!("{}({}, {}, {}, {})", 
        ctx.bootstrap.match_fn, 
        self.matched, ))
    }
}

impl JSCompilable for Vec<ASTAttribute> {
    fn compile(&self, _ctx: &mut CompilerContext) -> StroytellResult<String> {
        Ok(format!("[{}]", 
        self.iter().map(|i| 
            format!("{{name: {}, params: {}}}", i.name.safe_compile(), i.parameters.iter().map(|i| 
                i.safe_compile()).collect::<Vec<String>>().join(",")
            )).collect::<Vec<String>>().join(",")
        ))
    }
}

impl JSSafeCompilable for String {
    fn safe_compile(&self) -> String {
        format!("\"{}\"", self)
    }
}