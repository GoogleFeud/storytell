
//use storytell_parser::ast::model::*;

use super::project::CompilerContext;

pub trait HTMLCompilable {
    fn compile_html<CTX: CompilerContext>(&self, ctx: &mut CTX) -> String;
}

/* 
impl HTMLCompilable for ASTText {
    fn compile_html<CTX: CompilerContext>(&self, ctx: &mut CTX) -> String {
        if self.parts.is_empty() {
            return self.tail.clone()
        }
        let mut result = String::new();
        for part in &self.parts {
            result.push_str(&part.before);
            result.push_str(&part.text.compile_html(ctx))
        }
        result.push_str(&self.tail);
        result
    }
}

impl HTMLCompilable for ASTInline {
    fn compile_html<CTX: CompilerContext>(&self, ctx: &mut CTX) -> String {
        match self.kind {
            ASTInlineKind::Bold(text) => format!("<b>{}</b>", text.compile_html(ctx)),
            ASTInlineKind::Underline(text) => format!()
        }
    }
}
*/