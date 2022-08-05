
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

}


