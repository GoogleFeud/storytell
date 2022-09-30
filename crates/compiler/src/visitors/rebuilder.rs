use storytell_diagnostics::{location::Range, diagnostic::StorytellResult};
use storytell_js_parser::{ast::*, input::InputPresenter, JsParser};
use std::fmt::Write as _;

pub struct Rebuilder<'a> {
    input: InputPresenter<'a>,
    ident_prefix: Option<String>
}

impl<'a> Rebuilder<'a> {

    pub fn run(input: InputPresenter<'a>, prefix_ints: Option<String>, exps: &[ASTExpression]) -> String {
        let rebuilder = Rebuilder {
            ident_prefix: prefix_ints,
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

    fn stringify_exp(&self, exp: &ASTExpression) -> String {
        match exp {
            ASTExpression::String(str) => format!("\\\"{}\\\"", self.input.from_range(&Range::new(str.range.start + 1, str.range.end - 1))),
            ASTExpression::Number(num) => self.input.from_range(&num.range).to_string(),
            ASTExpression::Boolean(bool) => self.input.from_range(&bool.range).to_string(),
            ASTExpression::Identifier(ident) => {
                if let Some(prefix) = &self.ident_prefix {
                    format!("{}.{}", prefix, self.input.from_range(&ident.range))
                } else {
                    self.input.from_range(&ident.range).to_string()
                }
            },
            ASTExpression::Binary(binary) => format!("{}{}{}", self.stringify_exp(&binary.left), binary.operator, self.stringify_exp(&binary.right)),
            ASTExpression::Unary(unary) => format!("{}{}", unary.operator, self.stringify_exp(&unary.expression)),
            ASTExpression::Access(access) => {
                let str_exp = if let Some(prefix) = &self.ident_prefix {
                    match &access.expression {
                        ASTExpression::Identifier(ident) => format!("{}.{}", prefix, self.input.from_range(&ident.range)),
                        _ => self.stringify_exp(&access.expression)
                    }
                } else {
                    self.stringify_exp(&access.expression)
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

pub fn transform_js(input: &str, prefix_ints: Option<String>) -> StorytellResult<String> {
    let (result, diagnostics, input) = JsParser::parse(input);
    if diagnostics.is_empty() {
        Ok(Rebuilder::run(input, prefix_ints, &result))
    } else {
        Err(diagnostics)
    }
}