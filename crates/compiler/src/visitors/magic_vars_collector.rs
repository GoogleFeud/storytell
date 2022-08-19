use storytell_diagnostics::{diagnostic::Diagnostic, make_diagnostics, dia, location::Range };
use storytell_js_parser::{ast::*, tokenizer::TokenKind, input::InputPresenter};
use crate::visitors::flatten_access::flatten_access;
use std::{collections::HashMap, fmt::Display};

make_diagnostics!(define [
    MUST_BE_OBJ,
    C2001,
    "Variable $ is a $, not an object."
]);

#[derive(Clone, Debug)]
pub enum MagicVariableType {
    String,
    Number,
    Bool,
    Array,
    ObjectRef(u32),
    Unknown
}

impl MagicVariableType {
    pub fn get_id(&self) -> u8 {
        match self {
            Self::String => 0,
            Self::Number => 1,
            Self::Bool => 2,
            Self::Array => 3,
            Self::ObjectRef(_) => 4,
            Self::Unknown => 5
        }
    }
}

impl Display for MagicVariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Bool => write!(f, "boolean"),
            Self::Number => write!(f, "number"),
            Self::Array => write!(f, "array"),
            Self::ObjectRef(_) => write!(f, "object"),
            Self::Unknown => write!(f, "unknown")
        }
    }
}

pub type MagicObject = HashMap<String, MagicVariableType>;

pub struct MagicVariableCollectorContext {
    pub variables: MagicObject,
    pub objects: HashMap<u32, MagicObject>,
    pub diagnostics: Vec<Diagnostic>,
    pub counter: u32
}

impl Default for MagicVariableCollectorContext {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            objects: HashMap::new(),
            diagnostics: vec![],
            counter: 0
        }
    }
}

impl MagicVariableCollectorContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_obj(&mut self) -> MagicVariableType {
        let id = self.counter;
        self.counter += 1;
        self.objects.insert(id, HashMap::new());
        MagicVariableType::ObjectRef(id)
    }

    pub fn get_obj(&mut self, typ: &MagicVariableType) -> Option<&mut MagicObject> {
        if let MagicVariableType::ObjectRef(id) = typ {
            self.objects.get_mut(&id)
        } else {
            None
        }
    }

    pub fn get_or_create_chain(&mut self, chain: &[String]) -> &mut MagicObject {
        let mut store = &mut self.variables;
        for name in chain {
            if let Some(next_store) = store.get_mut(name) {
                if let MagicVariableType::ObjectRef(id) = next_store {
                    store = self.objects.get_mut(id)
                } else {
                    self.diagnostics.push(dia!(MUST_BE_OBJ, Range::default(), ))
                }
            } else {
                
            }
        }
    }

}

pub struct MagicVarCollector<'a> {
    pub input: InputPresenter<'a>,
    pub collected: Vec<(String, u8)>,
    pub ctx: &'a mut MagicVariableCollectorContext
}

impl<'a> MagicVarCollector<'a> {
    pub fn new(input: InputPresenter<'a>, ctx: &'a mut MagicVariableCollectorContext) -> Self {
        Self {
            collected: vec![],
            ctx,
            input
        }
    }
}

impl<'a> MagicVarCollector<'a> {

    fn process_exp(&mut self, exp: &ASTExpression) -> MagicVariableType {
        match exp {
            ASTExpression::Binary(exp) => {
                match &exp.left {
                    ASTExpression::Identifier(left_ident) => {
                        let var_name = self.input.from_range(&left_ident.range).to_string();
                        match exp.operator {
                            TokenKind::PlusEqualsOp | TokenKind::EqualsEqualsOp | TokenKind::EqualsEqualsEqualsOp | TokenKind::NotEqualsEqualsOp | TokenKind::EqualsOp => {
                                let variable_type = self.process_exp(&exp.right);
                                self.collected.push((var_name.clone(), variable_type.get_id()));
                                self.ctx.variables.insert(var_name, variable_type.clone());
                                variable_type
                            },
                            TokenKind::MinusEqualsOp | TokenKind::StarEqualsOp | TokenKind::StarEqualsOp if matches!(exp.right, ASTExpression::Number(_)) => {
                                self.collected.push((var_name.clone(), MagicVariableType::Number.get_id()));
                                self.ctx.variables.insert(var_name, MagicVariableType::Number);
                                MagicVariableType::Number
                            },
                            _ => {
                                let exp_type = self.process_exp(&exp.right);
                                self.collected.push((var_name.clone(), exp_type.get_id()));
                                self.ctx.variables.insert(self.input.from_range(&left_ident.range).to_string(), exp_type.clone());
                                exp_type
                            }
                        }
                    },
                    ASTExpression::Access(access) => {
                        if let Some(mut flattened) = flatten_access(&self.input, access) {
                            let last = flattened.pop().unwrap();
                            
                        }
                    }
                    _ => {
                        exp.visit_each_child(self);
                        MagicVariableType::Unknown
                    }
                }
            },
            ASTExpression::String(_) => MagicVariableType::String,
            ASTExpression::Number(_) => MagicVariableType::Number,
            ASTExpression::Boolean(_) => MagicVariableType::Bool,
            ASTExpression::ArrayLit(_) => MagicVariableType::Array,
            ASTExpression::Identifier(ident) => {
                if let Some(typ) = self.ctx.variables.get(self.input.from_range(&ident.range)) {
                    typ.clone()
                } else {
                    MagicVariableType::Unknown
                }
            },
            ASTExpression::Access(access) => {
                if let MagicVariableType::Object(obj) = self.process_exp(&access.expression) {
                    let right_text = match &access.accessor {
                        ASTAccessContent::Identifier(ident) => self.input.from_range(&ident.range),
                        ASTAccessContent::Expression(exp) => {
                            match exp {
                                ASTExpression::String(str) => self.input.from_range(&str.range),
                                ASTExpression::Number(num) => self.input.from_range(&num.range),
                                _ => return MagicVariableType::Unknown
                            }
                        }
                    };
                    obj.get(right_text).unwrap_or(&MagicVariableType::Unknown).clone()
                } else {
                    MagicVariableType::Unknown
                }
            },
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