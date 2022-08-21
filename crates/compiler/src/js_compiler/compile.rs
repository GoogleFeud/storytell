use crate::visitors::{MagicVarCollector, Rebuilder};
use super::{CompilerContext, Path};
use storytell_diagnostics::location::Range;
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
            ASTInlineKind::Code(text) => Ok(format!("\\`{}\\`", text.compile(ctx)?)),
            ASTInlineKind::Javascript(text) => {
                let (expressions, diagnostics, input) = JsParser::parse(text);
                if !diagnostics.is_empty() {
                    Err(diagnostics)
                } else {
                    let mut magic_vars_collector = MagicVarCollector::new(input, Range::new(self.range.start + 1, self.range.end - 1), &mut ctx.magic_variables);
                    expressions.visit_each_child(&mut magic_vars_collector);
                    if !magic_vars_collector.diagnostics.is_empty() {
                        Err(magic_vars_collector.diagnostics)
                    } else {
                        let gathered_variables = magic_vars_collector.collected.iter().map(|pair| format!("{{name: {}, type: {}}}", pair.0.safe_compile(), pair.1)).collect::<Vec<String>>();
                        let rebuilt_code = Rebuilder::run(magic_vars_collector.input, &expressions);
                        Ok(format!("${{{}({},{})}}", ctx.bootstrap.inline_js_fn, rebuilt_code.safe_compile(), gathered_variables.safe_compile()))
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
        let path_fn = ctx.bootstrap.path_fn;
        Ok(format!("{}({{title:{},canonicalTitle:{},childPaths:{{{}}},children:[{}]}})", 
        path_fn,
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
            return Ok(format!("{}(`{}`,{})", para, self.tail, self.attributes.compile(ctx)?))
        }
        let mut result = String::new();
        for part in &self.parts {
            result.push_str(&part.before);
            result.push_str(&part.text.compile(ctx)?)
        }
        result.push_str(&self.tail);
        Ok(format!("{}(`{}`,{})", para, result, self.attributes.compile(ctx)?))
    }
}

impl JSCompilable for ASTCodeBlock {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        let codeblock_fn = ctx.bootstrap.codeblock_fn;
        Ok(format!("{}(`{}`,{},{})", codeblock_fn, self.text.replace('`', "\\`"), self.language.safe_compile(), self.attributes.compile(ctx)?))
    }
}

impl JSCompilable for ASTDivert {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        match ctx.paths.try_get_child_by_path(&self.path) {
            Ok(_) => {
                Ok(format!("{}([{}])", ctx.bootstrap.divert_fn, self.path.iter().map(|string| format!("\"{}\"", string)).collect::<Vec<String>>().join(", ")))
            },
            Err(ind) => {
                if ind == 0 {
                    Err(vec![dia!(UNKNOWN_PATH, self.range.clone(), &self.path[ind])])
                } else {
                    Err(vec![dia!(UNKNOWN_CHILD_PATH, self.range.clone(), &self.path[ind], &self.path[ind - 1])])
                }
            }
        }
    }
}

impl JSCompilable for ASTMatch {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        let match_fn = ctx.bootstrap.match_fn;
        let mut choices: Vec<String> = vec![];
        for choice in &self.choices {
            choices.push(format!("{{text:{}, children:{}}}", transform_js(&choice.text.parts[0].text.to_raw())?.safe_compile(), choice.children.compile(ctx)?));
        }
        Ok(format!("{}({},{},{},{})", 
        match_fn, 
        transform_js(&self.matched)?.safe_compile(),
        choices.safe_compile(),
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
        Ok(format!("{{text:{},children:{}}}", self.text.compile(ctx)?.safe_compile(), self.children.compile(ctx)?))
    }
}

impl JSCompilable for ASTBlock {
    fn compile(&self, ctx: &mut CompilerContext) -> StorytellResult<String> {
        match self {
            Self::ChoiceGroup(block) => block.compile(ctx),
            Self::CodeBlock(block) => block.compile(ctx),
            Self::Header(block) => block.compile(ctx),
            Self::Match(block) => block.compile(ctx),
            Self::Paragraph(block) => block.compile(ctx),
            Self::Divert(divert) => divert.compile(ctx)
        }
    }
}

impl JSCompilable for Vec<ASTAttribute> {
    fn compile(&self, _ctx: &mut CompilerContext) -> StorytellResult<String> {
        Ok(format!("[{}]", 
        self.iter().map(|i|
            format!("{{name:{},params:[{}]}}", i.name.safe_compile(), i.parameters.iter().map(|i| format!("\"{}\"", i)).collect::<Vec<String>>().join(","))
        ).collect::<Vec<String>>().join(",")))
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

fn transform_js(input: &str) -> StorytellResult<String> {
    let (result, diagnostics, input) = JsParser::parse(input);
    if diagnostics.is_empty() {
        Ok(Rebuilder::run(input, &result))
    } else {
        Err(diagnostics)
    }
}