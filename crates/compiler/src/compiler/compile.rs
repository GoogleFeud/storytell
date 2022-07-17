use super::compiler::CompilerContext;
use storytell_parser::ast::model::*;
use storytell_diagnostics::diagnostic::*;

pub trait Compilable {
    fn compile(&self, ctx: &mut CompilerContext) -> StroytellResult<String>;
}

impl Compilable for ASTInline {
    fn compile(&self, ctx: &mut CompilerContext) -> StroytellResult<String> {
        match &self.kind {
            ASTInlineKind::Bold(text) => Ok(format!("<b>{}</b>", text.compile(ctx)?)),
            ASTInlineKind::Italics(text) => Ok(format!("<i>{}</i>", text.compile(ctx)?)),
            ASTInlineKind::Underline(text) => Ok(format!("<u>{}</u>", text.compile(ctx)?)),
            ASTInlineKind::Code(text) => Ok(format!("<code>{}</code>", text.compile(ctx)?)),
            ASTInlineKind::Javascript(text) => Ok(format!("${{{}}}", text)),
            ASTInlineKind::Divert(thing) => Ok(format!("${{{}}}", format!("{}([{}])", ctx.bootstrap.divert_fn, thing.iter().map(|string| format!("\"{}\"", string)).collect::<Vec<String>>().join(", ")))),
            ASTInlineKind::TempDivert(thing) => Ok(format!("${{{}}}", format!("{}([{}])", ctx.bootstrap.temp_divert_fn, thing.iter().map(|string| format!("\"{}\"", string)).collect::<Vec<String>>().join(", "))))
        }
    }
}

impl Compilable for ASTText {
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