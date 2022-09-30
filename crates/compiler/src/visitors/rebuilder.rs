use storytell_diagnostics::{location::Range, diagnostic::StorytellResult};
use storytell_js_parser::{ast::*, input::InputPresenter, JsParser};
use std::fmt::Write as _;

/// Receives JSParsed content, returns a **valid**
/// javascript string which can be evaled.
pub struct Rebuilder<'a> {
    input: InputPresenter<'a>,
    /// Prefixes all global identifiers with a property access,
    /// so for example if this property is set to "this",
    /// All variables will have `this.` before them.
    /// This excludes variables like "window", "document" and "location"
    prefix_idents: Option<String>
}

impl<'a> Rebuilder<'a> {
    pub fn run(input: InputPresenter<'a>, exps: &[ASTExpression], prefix_idents: Option<String>) -> String {
        let rebuilder = Rebuilder {
            prefix_idents,
            input
        };
        let mut output: Vec<String> = vec![];
        for exp in exps {
           output.push(rebuilder.stringify_exp(exp));
        }
        output.join(";")
    }

    fn stringify_vec_of_expr(&self, vector: &[ASTExpression]) -> String {
        vector.iter().map(|el| self.stringify_exp(el)).collect::<Vec<String>>().join(",")
    }

    fn resolve_ident(&self, range: &Range<usize>) -> String {
        let ident_text = self.input.from_range(range);
        match &self.prefix_idents {
            Some(prefix) if ident_text != prefix && ident_text != "window" && ident_text != "location" && ident_text != "document" => format!("{}.{}", prefix, ident_text),
            _ => ident_text.to_string()
        }
    }

    fn stringify_exp(&self, exp: &ASTExpression) -> String {
        match exp {
            ASTExpression::String(str) => format!("\\\"{}\\\"", self.input.from_range(&Range::new(str.range.start + 1, str.range.end - 1))),
            ASTExpression::Number(num) => self.input.from_range(&num.range).to_string(),
            ASTExpression::Boolean(bool) => self.input.from_range(&bool.range).to_string(),
            ASTExpression::Identifier(ident) => self.resolve_ident(&ident.range),
            ASTExpression::Binary(binary) => format!("{}{}{}", self.stringify_exp(&binary.left), binary.operator, self.stringify_exp(&binary.right)),
            ASTExpression::Unary(unary) => format!("{}{}", unary.operator, self.stringify_exp(&unary.expression)),
            ASTExpression::Access(access) => {
                let str_exp = match &access.expression {
                    ASTExpression::Identifier(ident) => self.resolve_ident(&ident.range),
                    _ => self.stringify_exp(&access.expression)
                };
                match &access.accessor {
                    ASTAccessContent::Expression(exp) => format!("{}[{}]", str_exp, self.stringify_exp(exp)),
                    ASTAccessContent::Identifier(ident) => format!("{}.{}", str_exp, self.input.from_range(&ident.range))
                }
            },
            ASTExpression::ArrayLit(lit) => format!("[{}]", self.stringify_vec_of_expr(&lit.elements)),
            ASTExpression::New(new) => format!("new {}({})", self.stringify_exp(&new.expression), self.stringify_vec_of_expr(&new.arguments)),
            ASTExpression::Call(call) => format!("{}({})", self.stringify_exp(&call.expression), self.stringify_vec_of_expr(&call.arguments)),
            ASTExpression::Ternary(ternary) => format!("{}?{}:{}", self.stringify_exp(&ternary.condition), self.stringify_exp(&ternary.left), self.stringify_exp(&ternary.right)),
            ASTExpression::StringTemplate(temp) => {
                let mut spans = String::new();
                for span in &temp.spans {
                    write!(spans, "{}${{{}}}", self.input.from_range(&span.before), self.stringify_exp(&span.expression)).unwrap();
                }
                format!("`{}{}`", spans, self.input.from_range(&temp.tail))
            }
        }
    }

}

pub fn transform_js(input: &str, prefix_idents: Option<String>) -> StorytellResult<String> {
    let (result, diagnostics, input) = JsParser::parse(input);
    if diagnostics.is_empty() {
        Ok(Rebuilder::run(input, &result, prefix_idents))
    } else {
        Err(diagnostics)
    }
}