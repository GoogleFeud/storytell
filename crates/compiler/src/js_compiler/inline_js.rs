
use storytell_js_parser::{ast::*, tokenizer::TokenKind, input::InputPresenter};
use crate::js_compiler::MagicVariableType;
use std::collections::HashMap;

pub struct MagicVariableTraverser<'a> {
    pub input: InputPresenter<'a>,
    pub magic_variables: HashMap<String, MagicVariableType>
}

impl<'a> MagicVariableTraverser<'a> {
    pub fn new(input: InputPresenter<'a>) -> Self {
        Self { 
            magic_variables: HashMap::new(),
            input
        }
    }
}

impl<'a> Visitor for MagicVariableTraverser<'a> {
    fn binary(&mut self, exp: &ASTBinary) {
        if let ASTExpression::Identifier(left_ident) = &exp.left {
            match exp.operator {
                TokenKind::PlusEqualsOp | TokenKind::EqualsEqualsOp | TokenKind::EqualsEqualsEqualsOp | TokenKind::NotEqualsEqualsOp => {
                    let variable_type = match exp.right {
                        ASTExpression::String(_) => MagicVariableType::String,
                        ASTExpression::Number(_) => MagicVariableType::Number,
                        ASTExpression::Boolean(_) => MagicVariableType::Bool,
                        _ => MagicVariableType::Unknown
                    };
                    self.magic_variables.insert(self.input.from_range(&left_ident.range).to_string(), variable_type);
                },
                TokenKind::MinusEqualsOp | TokenKind::StarEqualsOp | TokenKind::StarEqualsOp if is_num(&exp.right) => {
                    self.magic_variables.insert(self.input.from_range(&left_ident.range).to_string(), MagicVariableType::Number);
                },
                _ => {}
            }
        }
    }
}

pub fn is_str(exp: &ASTExpression) -> bool {
    match exp {
        ASTExpression::String(_) => true,
        _ => false
    }
}

pub fn is_num(exp: &ASTExpression) -> bool {
    match exp {
        ASTExpression::Number(_) => true,
        _ => false
    }
}