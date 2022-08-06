
use crate::{tokenizer::{Tokenizer, TokenKind}, input::InputPresenter};
use storytell_diagnostics::{diagnostic::*, *};
use self::ast::*;
pub mod ast;

make_diagnostics!(define [
    UNKNOWN_TOKEN,
    JSP2001,
    "Unknown token $."
]);

pub struct JsParser<'a> {
    pub tokens: Tokenizer<'a>,
    pub errors: Vec<Diagnostic>
}

impl<'a> JsParser<'a> {

    pub fn new(content: &'a str) -> Self {
        Self { 
            tokens: Tokenizer::new(content),
            errors: vec![]
        }
    }

    pub fn resolve_prec(token: &TokenKind) -> u8 {
        match token {
            TokenKind::StarOp | TokenKind::SlashOp | TokenKind::PercentOp => 12,
            TokenKind::PlusOp | TokenKind::MinusOp => 11,
            TokenKind::LessThanOp | TokenKind::LessThanEqualsOp | TokenKind::GreaterThanOp | TokenKind::GreaterThanEqualsOp => 9,
            TokenKind::EqualsEqualsOp | TokenKind::EqualsEqualsEqualsOp | TokenKind::NotEqualsOp | TokenKind::NotEqualsEqualsOp => 8,
            TokenKind::AmpersandAmpersandOp => 4,
            TokenKind::BarBarOp | TokenKind::QuestionQuestionOp => 3,
            TokenKind::PlusEqualsOp | TokenKind::MinusEqualsOp | TokenKind::SlashEqualsOp | TokenKind::StarEqualsOp | TokenKind::EqualsOp => 2,
            _ => 0
        }
    }

    fn parse_binary(&mut self, left: ASTExpression, left_prec: u8) -> Option<ASTExpression> {
        let start = left.range().start;
        if let Some(next) = self.tokens.peek() {
            let right_prec = Self::resolve_prec(&next.kind);
            if right_prec == 0 {
                return Some(left);
            };
            if right_prec > left_prec {
                let op_token = self.tokens.next().unwrap();
                let exp = if let Some(exp) = self.parse_single_expression() {
                    exp
                } else { 
                    return Some(left)
                };
                let right = self.parse_binary(exp, right_prec)?;
                self.parse_binary(ASTExpression::Binary(Box::from(ASTBinary {
                    left,
                    right,
                    operator: op_token.kind,
                    range: self.tokens.input.range(start)
                })), left_prec)
            } else {
                Some(left)
            }
        } else {
            Some(left)
        }
    }

    fn parse_single_expression(&mut self) -> Option<ASTExpression> {
        let token = self.tokens.next()?;
        match token.kind {
            TokenKind::String => Some(ASTExpression::String(ASTString { range: token.range })),
            TokenKind::Number => Some(ASTExpression::Number(ASTNumber { range: token.range })),
            TokenKind::Identifier => Some(ASTExpression::Identifier(ASTIdentifier { range: token.range })),
            TokenKind::FalseKeyword | TokenKind::TrueKeyword => Some(ASTExpression::Boolean(ASTBoolean { range: token.range })),
            _ => {
                self.errors.push(dia!(UNKNOWN_TOKEN, token.range, self.tokens.input.data.from_range(&token.range)));
                None
            }
        }
    }

    fn parse_full_expression(&mut self) -> Option<ASTExpression> {
        if let Some(exp) = self.parse_single_expression() {
            self.parse_binary(exp, 0)
        } else {
            None
        }
    }

    pub fn parse(content: &'a str) -> (Vec<ASTExpression>, Vec<Diagnostic>, Vec<Diagnostic>, InputPresenter<'a>) {
        let mut parser = JsParser::new(content);
        let mut result: Vec<ASTExpression> = vec![];
        while let Some(exp) = parser.parse_full_expression() {
            result.push(exp);
            if parser.tokens.is_next(TokenKind::SemicolonPunc) {
                parser.tokens.next();
            } else {
                break;
            }
        }
        (result, parser.errors, parser.tokens.errors, parser.tokens.input.data)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let (tokens, _, _, input) = JsParser::parse("\"HelloWorld\"; 123.4; false");
        assert_eq!(input.from_range(tokens[0].range()), "\"HelloWorld\"");
        assert_eq!(input.from_range(tokens[1].range()), "123.4");
        assert_eq!(input.from_range(tokens[2].range()), "false");
    }

    pub struct MyVisitor<'a> {
        occurance: u8,
        input: InputPresenter<'a>
    }

    impl<'a> Visitor for MyVisitor<'a> {
        fn binary(&mut self, exp: &ASTBinary) {
            println!("THING: {}", self.input.from_range(&exp.range));
            if self.occurance == 0 {
                assert_eq!(exp.operator, TokenKind::MinusOp);
            }
            if self.occurance == 1 {
                assert_eq!(self.input.from_range(exp.left.range()), "a");
                assert_eq!(self.input.from_range(exp.right.range()), "b");
            }
            if self.occurance == 2 {
                assert_eq!(self.input.from_range(exp.right.range()), "c");
                assert_eq!(exp.operator, TokenKind::StarOp);
            }
            if self.occurance == 3 {
                assert_eq!(self.input.from_range(exp.left.range()), "c");
                assert_eq!(self.input.from_range(exp.right.range()), "d");
            }
            self.occurance += 1;
            exp.visit_each_child(self);
        }
    }

    #[test]
    fn test_binary_prec() {
        let (tokens, _, _, input) = JsParser::parse("
            a + b - c / d * c
        ");
        let mut visitor = MyVisitor { input, occurance: 0 };
        tokens[0].visit(&mut visitor);
    }
}