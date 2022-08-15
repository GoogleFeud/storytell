use storytell_diagnostics::location::Range;
use storytell_js_parser::{ast::*, input::InputPresenter};

pub struct Rebuilder<'a> {
    input: InputPresenter<'a>
}

impl<'a> Rebuilder<'a> {
    pub fn run(input: InputPresenter<'a>, exps: &[ASTExpression]) -> String {
        let mut rebuilder = Rebuilder { input };
        let mut output: Vec<String> = vec![];
        for exp in exps {
           output.push(format!("{}", rebuilder.stringify_exp(exp)));
        }
        output.join(";")
    }

    fn stringify_vec_of_expr(&mut self, vector: &[ASTExpression]) -> String {
        vector.iter().map(|el| self.stringify_exp(el)).collect::<Vec<String>>().join(",")
    }

    fn stringify_exp(&mut self, exp: &ASTExpression) -> String {
        match exp {
            ASTExpression::String(str) => format!("\\\"{}\\\"", self.input.from_range(&Range::new(str.range.start + 1, str.range.end - 1))),
            ASTExpression::Number(num) => self.input.from_range(&num.range).to_string(),
            ASTExpression::Boolean(bool) => self.input.from_range(&bool.range).to_string(),
            ASTExpression::Identifier(ident) => self.input.from_range(&ident.range).to_string(),
            ASTExpression::Binary(binary) => format!("{}{}{}", self.stringify_exp(&binary.left), binary.operator, self.stringify_exp(&binary.right)),
            ASTExpression::Unary(unary) => format!("{}{}", unary.operator, self.stringify_exp(&unary.expression)),
            ASTExpression::Access(access) => {
                match &access.accessor {
                    ASTAccessContent::Expression(exp) => format!("{}[{}]", self.stringify_exp(&access.expression), self.stringify_exp(exp)),
                    ASTAccessContent::Identifier(ident) => format!("{}.{}", self.stringify_exp(&access.expression), self.input.from_range(&ident.range))
                }
            },
            ASTExpression::ArrayLit(lit) => format!("[{}]", self.stringify_vec_of_expr(&lit.elements)),
            ASTExpression::New(new) => format!("new {}({})", self.stringify_exp(&new.expression), self.stringify_vec_of_expr(&new.arguments)),
            ASTExpression::Call(call) => format!("{}({})", self.stringify_exp(&call.expression), self.stringify_vec_of_expr(&call.arguments)),
            ASTExpression::Ternary(ternary) => format!("{}?{}:{}", self.stringify_exp(&ternary.condition), self.stringify_exp(&ternary.left), self.stringify_exp(&ternary.right))
        }
    }

}