use storytell_diagnostics::{diagnostic::{Diagnostic, DiagnosticMessage}, make_diagnostics, location::Range };
use storytell_js_parser::{ast::*, tokenizer::TokenKind, input::InputPresenter};
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

#[derive(Clone, Debug)]
pub enum ResolveChainResult<'a> {
    Top(&'a str),
    Nested(u32, &'a str)
}

impl<'a> ResolveChainResult<'a> {
    pub fn get(&self, ctx: &'a MagicVariableCollectorContext) -> Option<&MagicVariableType> {
        match self {
            Self::Top(name) => ctx.variables.get(*name),
            Self::Nested(id, name) => ctx.objects.get(id)?.get(*name)
        }
    }
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

    pub fn create_obj(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        self.objects.insert(id, HashMap::new());
        id
    }

    pub fn get_obj(&mut self, typ: &MagicVariableType) -> Option<&mut MagicObject> {
        if let MagicVariableType::ObjectRef(id) = typ {
            self.objects.get_mut(&id)
        } else {
            None
        }
    }

    pub fn get_obj_id_from_name(&self, name: &str) -> Option<u32> {
        if let MagicVariableType::ObjectRef(id) = self.variables.get(name)? {
            Some(id.clone())
        } else {
            None
        } 
    }

}

pub struct MagicVarCollector<'a> {
    pub input: InputPresenter<'a>,
    pub collected: Vec<(String, u8)>,
    pub start_pos: Range<usize>,
    pub ctx: &'a mut MagicVariableCollectorContext
}

impl<'a> MagicVarCollector<'a> {
    pub fn new(input: InputPresenter<'a>, start_pos: Range<usize>, ctx: &'a mut MagicVariableCollectorContext) -> Self {
        Self {
            collected: vec![],
            start_pos,
            ctx,
            input
        }
    }
}

impl<'a> MagicVarCollector<'a> {

    fn get_string_from_accessor(&self, accessor: &ASTAccessContent) -> Option<&str> {
        match accessor {
            ASTAccessContent::Identifier(ident) => Some(self.input.from_range(&ident.range)),
            ASTAccessContent::Expression(exp) => match exp {
                ASTExpression::String(str) => Some(self.input.from_range(&str.range)),
                ASTExpression::Number(num) => Some(self.input.from_range(&num.range)),
                _ => None
            }
        }
    }

    fn resolve_chain(&mut self, chain: &ASTAccess) -> Option<ResolveChainResult> {
        if let ASTExpression::Access(_) = &chain.expression {
            let mut result = vec![];
            let mut left = &chain.expression;
            while let ASTExpression::Access(acc) = left {
                left = &acc.expression;
                result.push(self.get_string_from_accessor(&acc.accessor)?.to_string());
            }
            let first_object_name = if let ASTExpression::Identifier(ident) = left {
                self.input.from_range(&ident.range)
            } else {
                return None;
            };
            let mut store = if let Some(id) = self.ctx.get_obj_id_from_name(first_object_name) { id } else {
                let new_obj_id = self.ctx.create_obj();
                self.ctx.variables.insert(first_object_name.to_string(), MagicVariableType::ObjectRef(new_obj_id));
                new_obj_id
            };
            for object_name in result.iter().rev() {
                if let Some(obj) = self.ctx.objects.get(&store).unwrap().get(object_name) {
                    if let MagicVariableType::ObjectRef(id) = obj {
                        store = id.clone()
                    } else {
                        // Report that it's not an object
                    }
                } else {
                    let new_obj_id = self.ctx.create_obj();
                    self.ctx.objects.get_mut(&store).unwrap().insert(object_name.to_string(), MagicVariableType::ObjectRef(new_obj_id));
                    store = new_obj_id;
                }
            }
            Some(ResolveChainResult::Nested(store, self.get_string_from_accessor(&chain.accessor)?))
        } else {
            Some(ResolveChainResult::Top(self.get_string_from_accessor(&chain.accessor)?))
        }
    }

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
                        println!("{:?}", self.resolve_chain(access));
                        MagicVariableType::Unknown
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
            }
            ASTExpression::Access(access) => {
                println!("{:?}", self.resolve_chain(access));
                MagicVariableType::Unknown
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