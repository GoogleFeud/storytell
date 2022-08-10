use super::inline_js::MagicVariableTraverser;
use super::{CompilerContext, Path};
use storytell_js_parser::ast::Visitable;
use storytell_parser::ast::model::*;
use storytell_diagnostics::diagnostic::*;
use storytell_diagnostics::{dia, make_diagnostics};
use storytell_js_parser::JsParser;

make_diagnostics!(define [
    UNKNOWN_CHILD_PATH,
    C1001,
    "\"$\" is not a sub-path of \"$\"."
], [
    UNKNOWN_PATH,
    C1002,
    "\"$\" is not a path."
]);

pub trait JSCompilable {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String>;
}

pub trait JSSafeCompilable {
    fn safe_compile(&self) -> String { String::new() }
}

impl JSCompilable for ASTInline {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        match &self.kind {
            ASTInlineKind::Bold(text) => Ok(format!("<b>{}</b>", text.compile(ctx)?)),
            ASTInlineKind::Italics(text) => Ok(format!("<i>{}</i>", text.compile(ctx)?)),
            ASTInlineKind::Underline(text) => Ok(format!("<u>{}</u>", text.compile(ctx)?)),
            ASTInlineKind::Code(text) => Ok(format!("<code>{}</code>", text.compile(ctx)?)),
            ASTInlineKind::Javascript(text) => {
                let (expressions, diagnostics, input) = JsParser::parse(text);
                if !diagnostics.is_empty() {
                    Err(diagnostics)
                } else {
                    let mut visitor = MagicVariableTraverser::new(input);
                    expressions.visit_each_child(&mut visitor);
                    ctx.magic_variables.extend(visitor.magic_variables.into_iter());
                    Ok(format!("${{{}({})}}", ctx.bootstrap.inline_js_fn, text.safe_compile()))
                }
            },
            ASTInlineKind::Divert(thing, is_temp) => {
                match ctx.paths.try_get_child_by_path(thing) {
                    Ok(_) => {
                        Ok(format!("${{{}([{}])}}", if *is_temp { ctx.bootstrap.temp_divert_fn } else { ctx.bootstrap.divert_fn }, thing.iter().map(|string| format!("\"{}\"", string)).collect::<Vec<String>>().join(", ")))
                    },
                    Err(ind) => {
                        if ind == 0 {
                            Err(vec![dia!(UNKNOWN_PATH, self.range.clone(), &thing[ind])])
                        } else {
                            Err(vec![dia!(UNKNOWN_CHILD_PATH, self.range.clone(), &thing[ind], &thing[ind - 1])])
                        }
                    }
                }
            }
        }
    }
}

impl JSCompilable for ASTText {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
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
    ///     childPaths: {self},
    ///     children: []
    /// }
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
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
            childPaths: {{{}}},
            children: [{}]
        }}", 
        self.title.text.safe_compile(), 
        Path::canonicalize_name(&self.title.text).safe_compile(),
        header_children.join(","),
        others.iter().map(|i| i.compile(ctx)).collect::<StorytellResult<Vec<String>>>()?.join(",")
        ))
    }
}

impl JSCompilable for ASTParagraph {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        let para = ctx.bootstrap.paragraph_fn;
        if self.parts.is_empty() {
            return Ok(format!("{}({}, {})", para, self.tail.safe_compile(), self.attributes.compile(ctx)?))
        }
        let mut result = String::new();
        for part in &self.parts {
            result.push_str(&part.before);
            result.push_str(&part.text.compile(ctx)?)
        }
        result.push_str(&self.tail);
        Ok(format!("{}({}, {})", para, result.safe_compile(), self.attributes.compile(ctx)?))
    }
}

impl JSCompilable for ASTCodeBlock {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        let codeblock_fn = ctx.bootstrap.codeblock_fn;
        Ok(format!("{}({}, {}, {})", codeblock_fn, self.text.safe_compile(), self.language.safe_compile(), self.attributes.compile(ctx)?))
    }
}

impl JSCompilable for ASTMatch {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        let match_fn = ctx.bootstrap.match_fn;
        Ok(format!("{}({}, {}, {}, {})", 
        match_fn, 
        self.matched,
        self.choices.compile(ctx)?,
        self.direct_children.compile(ctx)?,
        self.kind.clone().unwrap_or_else(|| String::from("\"\""))
        ))
    }
}

impl JSCompilable for ASTChoiceGroup {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        let choice_group_fn = ctx.bootstrap.choice_group_fn;
        Ok(format!("{}({}, {})",
        choice_group_fn,
        self.choices.compile(ctx)?,
        self.attributes.compile(ctx)?
        ))
    }
}

impl JSCompilable for ASTChoice {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        Ok(format!("{{text: {}, children: {}}}", self.text.compile(ctx)?, self.children.compile(ctx)?))
    }
}

impl JSCompilable for ASTBlock {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        match self {
            Self::ChoiceGroup(block) => block.compile(ctx),
            Self::CodeBlock(block) => block.compile(ctx),
            Self::Header(block) => block.compile(ctx),
            Self::Match(block) => block.compile(ctx),
            Self::Paragraph(block) => block.compile(ctx)
        }
    }
}

impl JSCompilable for Vec<ASTAttribute> {
    fn compile(&self, _ctx: &mut CompilerContext) -> StorytellResult<String> {
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

impl JSSafeCompilable for Vec<String> {
    fn safe_compile(&self) -> String {
        format!("[{}]", self.join(","))
    }
}

impl<T: JSCompilable> JSCompilable for Vec<T> {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        Ok(format!("[{}]", self.iter().map(|i| i.compile(ctx)).collect::<StorytellResult<Vec<String>>>()?.join(",")))
    }
}
