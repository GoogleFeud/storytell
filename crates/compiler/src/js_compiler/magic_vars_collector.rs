use storytell_js_parser::{ast::*, tokenizer::TokenKind, input::InputPresenter};
use crate::js_compiler::MagicVariableType;
use std::collections::HashMap;

pub struct MagicVarCollector<'a> {
    pub input: InputPresenter<'a>,
    pub magic_variables: HashMap<String, MagicVariableType>
}

impl<'a> MagicVarCollector<'a> {
    pub fn new(input: InputPresenter<'a>) -> Self {
        Self { 
            magic_variables: HashMap::new(),
            input
        }
    }
}

impl<'a> MagicVarCollector<'a> {
    fn process_exp(&mut self, exp: &ASTExpression) -> MagicVariableType {
        match exp {
            ASTExpression::Binary(exp) => {
                if let ASTExpression::Identifier(left_ident) = &exp.left {
                    match exp.operator {
                        TokenKind::PlusEqualsOp | TokenKind::EqualsEqualsOp | TokenKind::EqualsEqualsEqualsOp | TokenKind::NotEqualsEqualsOp | TokenKind::EqualsOp => {
                            let variable_type = self.process_exp(&exp.right);
                            self.magic_variables.insert(self.input.from_range(&left_ident.range).to_string(), variable_type);
                            variable_type
                        },
                        TokenKind::MinusEqualsOp | TokenKind::StarEqualsOp | TokenKind::StarEqualsOp if matches!(exp.right, ASTExpression::Number(_)) => {
                            self.magic_variables.insert(self.input.from_range(&left_ident.range).to_string(), MagicVariableType::Number);
                            MagicVariableType::Number
                        },
                        _ => {
                            let exp_type = self.process_exp(&exp.right);
                            self.magic_variables.insert(self.input.from_range(&left_ident.range).to_string(), exp_type);
                            exp_type
                        }
                    }
                } else {
                    exp.visit_each_child(self);
                    MagicVariableType::Unknown
                }
            },
            ASTExpression::String(_) => MagicVariableType::String,
            ASTExpression::Number(_) => MagicVariableType::Number,
            ASTExpression::Boolean(_) => MagicVariableType::Bool,
            ASTExpression::ArrayLit(_) => MagicVariableType::Array,
            _ => {
                exp.visit_each_child(self);
                MagicVariableType::Unknown
            }
        }
    }
}

impl<'a> Visitor for MagicVarCollector<'a> {
    fn expression(&mut self,exp: &ASTExpression) {
        self.process_exp(exp);
    }
}