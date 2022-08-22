use storytell_diagnostics::{diagnostic::*, make_diagnostics, location::Range, dia };
use storytell_js_parser::{ast::*, tokenizer::{TokenKind}, input::InputPresenter};
use std::{collections::HashMap, fmt::Display};

make_diagnostics!(define [
    MUST_BE_OBJ,
    C2001,
    "Variable '$' is a '$', not an object."
], [
    DIFFERENT_TYPE,
    C2002,
    "Variable '$' is a '$', but '$' is being assigned to it."
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
pub enum ResolveChainResult {
    Top(String),
    Nested(u32, String),
    None
}

impl ResolveChainResult {
    pub fn get<'a>(&self, ctx: &'a MagicVariableCollectorContext) -> Option<&'a MagicVariableType> {
        match self {
            Self::Top(name) => ctx.variables.get(name),
            Self::Nested(id, name) => ctx.objects.get(id)?.get(name),
            Self::None => None
        }
    }

    pub fn get_store<'a>(&self, ctx: &'a mut MagicVariableCollectorContext) -> Option<(&'a mut MagicObject, &String)> {
        match self {
            Self::Top(name) => Some((&mut ctx.variables, name)),
            Self::Nested(id, name) => Some((ctx.objects.get_mut(id)?, name)),
            Self::None => None
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

#[derive(Debug, Default)]
pub struct MagicVariableCollectorContext {
    pub variables: MagicObject,
    pub objects: HashMap<u32, MagicObject>,
    pub counter: u32
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
            self.objects.get_mut(id)
        } else {
            None
        }
    }

    pub fn get_obj_id_from_name(&self, name: &str) -> Option<u32> {
        if let MagicVariableType::ObjectRef(id) = self.variables.get(name)? {
            Some(*id)
        } else {
            None
        } 
    }

}

pub struct MagicVarCollector<'a> {
    pub input: InputPresenter<'a>,
    pub collected: Vec<(String, u8)>,
    pub start_pos: Range<usize>,
    pub diagnostics: Vec<Diagnostic>,
    pub ctx: &'a mut MagicVariableCollectorContext
}

impl<'a> MagicVarCollector<'a> {
    pub fn new(input: InputPresenter<'a>, start_pos: Range<usize>, ctx: &'a mut MagicVariableCollectorContext) -> Self {
        Self {
            collected: vec![],
            diagnostics: vec![],
            start_pos,
            ctx,
            input
        }
    }
}

impl<'a> MagicVarCollector<'a> {

    fn range(&self, other: &Range<usize>) -> Range<usize> {
        let start = self.start_pos.start + other.start;
        Range { 
            start,
            end: start + other.end
        }
    }

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

    fn resolve_chain(&mut self, chain: &ASTAccess) -> ResolveChainResult {
        if let ASTExpression::Access(_) = &chain.expression {
            let mut result = vec![];
            let mut left = &chain.expression;
            while let ASTExpression::Access(acc) = left {
                left = &acc.expression;
                result.push((match self.get_string_from_accessor(&acc.accessor) {
                    Some(val) => val.to_string(),
                    None => return ResolveChainResult::None
                }, &acc.range));
            }
            let (first_object_name, first_object_name_range) = if let ASTExpression::Identifier(ident) = left {
                (self.input.from_range(&ident.range), &ident.range)
            } else {
                return ResolveChainResult::None;
            };
            let mut store = if let Some(var_type) = self.ctx.variables.get(first_object_name) {
                if let MagicVariableType::ObjectRef(id) = var_type {
                    *id
                } else {
                    self.diagnostics.push(dia!(MUST_BE_OBJ, self.range(first_object_name_range), first_object_name, &var_type.to_string()));
                    return ResolveChainResult::None;
                }
            } else {
                let new_obj_id = self.ctx.create_obj();
                self.ctx.variables.insert(first_object_name.to_string(), MagicVariableType::ObjectRef(new_obj_id));
                self.collected.push((first_object_name.to_string(), 4));
                new_obj_id 
            };
            for (object_name, object_range) in result.iter().rev() {
                if let Some(obj) = self.ctx.objects.get(&store).unwrap().get(object_name) {
                    if let MagicVariableType::ObjectRef(id) = obj {
                        store = *id;
                    } else {
                        self.diagnostics.push(dia!(MUST_BE_OBJ, self.range(object_range), object_name, &obj.to_string()));
                    }
                } else {
                    let new_obj_id = self.ctx.create_obj();
                    self.ctx.objects.get_mut(&store).unwrap().insert(object_name.to_string(), MagicVariableType::ObjectRef(new_obj_id));
                    store = new_obj_id;
                }
            }
            ResolveChainResult::Nested(store, match self.get_string_from_accessor(&chain.accessor) {
                    Some(val) => val.to_string(),
                    None => return ResolveChainResult::None
                })
        } else {
            ResolveChainResult::Top(match self.get_string_from_accessor(&chain.accessor) {
                Some(val) => val.to_string(),
                None => return ResolveChainResult::None
            })
        }
    }

    fn resolve_binary(&mut self, op: &TokenKind, right: &ASTExpression) -> MagicVariableType {
        match op {
            TokenKind::MinusEqualsOp | TokenKind::StarEqualsOp | TokenKind::StarEqualsOp if matches!(right, ASTExpression::Number(_)) => MagicVariableType::Number,
            _ => self.process_exp(right)
        }
    }

    fn process_exp(&mut self, exp: &ASTExpression) -> MagicVariableType {
        match exp {
            ASTExpression::Binary(exp) => {
                match &exp.left {
                    ASTExpression::Identifier(left_ident) => {
                        let left_name = self.input.from_range(&left_ident.range).to_string();
                        let var_type = self.resolve_binary(&exp.operator, &exp.right);
                        self.collected.push((left_name.clone(), var_type.get_id()));
                        if let Some(prev) = self.ctx.variables.insert(left_name.to_string(), var_type.clone()) {
                            if !matches!(prev, MagicVariableType::Unknown) {
                                self.diagnostics.push(dia!(DIFFERENT_TYPE, self.range(&exp.range), &left_name, &prev.to_string(), &var_type.to_string()))
                            }
                        }
                        var_type
                    },
                    ASTExpression::Access(access) => {
                        let right_type = self.resolve_binary(&exp.operator, &exp.right);
                        if let Some((store, var_name)) = self.resolve_chain(access).get_store(self.ctx) {
                            if let Some(prev) = store.insert(var_name.clone(), right_type.clone()) {
                                if !matches!(prev, MagicVariableType::Unknown) {
                                    self.diagnostics.push(dia!(DIFFERENT_TYPE, self.range(&exp.range), var_name, &prev.to_string(), &right_type.to_string()))
                                }
                            }
                        }
                        right_type
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
               if let Some(var_type) = self.resolve_chain(access).get(self.ctx) {
                    var_type.clone()
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