use super::compiler::CompilerContext;
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
            ASTInlineKind::Divert(thing, is_temp) => {
                match ctx.paths.try_get_child_by_path(&thing) {
                    Ok(_) => {
                        Ok(format!("${{{}}}", format!("{}([{}])", if *is_temp { ctx.bootstrap.temp_divert_fn } else { ctx.bootstrap.divert_fn }, thing.iter().map(|string| format!("\"{}\"", string)).collect::<Vec<String>>().join(", "))))
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