
use crate::tokenizer::{Tokenizer, TokenKind};
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
        let start = self.tokens.input.pos;
        if let Some(next) = self.tokens.peek() {
            let right_prec = Self::resolve_prec(&next.kind);
            if right_prec == 0 {
                return Some(left);
            };
            if right_prec > left_prec {
                let op_token = self.tokens.next().unwrap();
                let exp = if let Some(exp) = self.parse_single_expression() {
                    exp
                } else { return Some(left) };
                let right = self.parse_binary(exp, right_prec)?;
                Some(ASTExpression::Binary(Box::from(ASTBinary {
                    left: left,
                    right,
                    operator: op_token.kind,
                    range: self.tokens.input.range(start)
                })))
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
        unimplemented!("Not implemented")
    }

}


