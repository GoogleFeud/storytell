use super::compiler::CompilerContext;
use storytell_parser::ast::model::*;

pub trait Compilable<CTX: CompilerContext> {
    fn compile(&self, ctx: &mut CTX) -> String;
}

impl<T: CompilerContext> Compilable<T> for ASTInline {
    fn compile(&self, ctx: &mut T) -> String {
        match self.kind {
            ASTInlineKind::Bold(text) => format!("<b>{}</b>", text.compile(ctx)),
            ASTInlineKind::Italics(text) => format!("<i>{}</i>", text.compile(ctx)),
            ASTInlineKind::Underline(text) => format!("<u>{}</u>", text.compile(ctx)),
            ASTInlineKind::Code(text) => format!("<code>{}</code>", text.compile(ctx)),
            ASTInlineKind::Javascript(text) => format!("${{{}}}", text),
            ASTInlineKind::Divert(thing) => format!("${}")
        }
    }
}

impl<T: CompilerContext> Compilable<T> for ASTText {
    fn compile(&self, ctx: &mut T) -> String {
        if self.parts.is_empty() {
            return self.tail.clone()
        }
        let mut result = String::new();
        for part in &self.parts {
            result.push_str(&part.before);
            result.push_str(&part.text.compile(ctx))
        }
        result.push_str(&self.tail);
        result
    }
}