pub mod magic_var;
use magic_var::{MagicObject, MagicVariableType};
use rustc_hash::FxHashMap;
use std::{hash::Hash};
use storytell_diagnostics::location::Range;
use storytell_js_parser::{
    ast::{ASTExpression, Visitable, Visitor, ASTAccess, ASTAccessContent},
    input::InputPresenter,
    tokenizer::TokenKind,
};

use self::magic_var::{MagicVariableInstance, MagicVariable};

#[derive(Debug, Default)]
pub struct MagicVariableCollectorContext<ORIGIN: Eq + Hash + Default> {
    pub variables: MagicObject<ORIGIN>,
    pub objects: FxHashMap<u32, MagicObject<ORIGIN>>,
    pub counter: u32,
}

impl<ORIGIN: Eq + Hash + Default + Copy> MagicVariableCollectorContext<ORIGIN> {
    pub fn create_obj(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        self.objects.insert(id, MagicObject::default());
        id
    }

    pub fn remove_from_origin(&mut self, origin: &ORIGIN) {
        for object in self.variables.0.values_mut().chain(
            self.objects
                .values_mut()
                .flat_map(|v| v.0.values_mut())
        ) {
            object.0.retain(|v| v.origin != *origin)
        }
    }
}

pub struct MagicVariableCollector<'a, ORIGIN: Eq + Hash + Default + Copy> {
    pub origin: ORIGIN,
    pub ctx: &'a mut MagicVariableCollectorContext<ORIGIN>,
    pub input: InputPresenter<'a>,
    pub start_pos: Range<usize>,
}

impl<'a, ORIGIN: Eq + Hash + Default + Copy> MagicVariableCollector<'a, ORIGIN> {

    fn range(&self, other: &Range<usize>) -> Range<usize> {
        let start = self.start_pos.start + other.start;
        Range {
            start,
            end: start + other.end,
        }
    }

    fn resolve_binary(&mut self, op: &TokenKind, right: &ASTExpression) -> MagicVariableType {
        match op {
            TokenKind::MinusEqualsOp | TokenKind::StarEqualsOp | TokenKind::StarEqualsOp
                if matches!(right, ASTExpression::Number(_)) => {
                MagicVariableType::Number
            }
            _ => self.process_exp(right),
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

    fn resolve_access(&mut self, access: &ASTAccess) -> Option<(u32, String)> {
        let mut result = vec![];
        let mut left = &access.expression;
        while let ASTExpression::Access(acc) = left {
            left = &acc.expression;
            result.push((match self.get_string_from_accessor(&acc.accessor) {
                Some(val) => val.to_string(),
                None => return None
            }, acc.range.clone()));
        };
        let (first_obj, first_range) = if let ASTExpression::Identifier(ident) = left {
            (self.input.from_range(&ident.range), ident.range.clone())
        } else {
            return None;
        };
        let mut store = if let Some(var_type) = self.ctx.variables.0.get_mut(first_obj) {
            if let MagicVariableType::ObjectRef(id) = var_type.get_common_type().unwrap_or(MagicVariableType::Unknown) {
                id
            } else {
                let obj_ref = self.ctx.create_obj();
                var_type.insert(MagicVariableInstance {
                    origin: self.origin,
                    value: MagicVariableType::ObjectRef(obj_ref),
                    range: self.range(&first_range)
                });
                obj_ref
            }
        } else {
            let obj_ref = self.ctx.create_obj();
            self.ctx.variables.insert(first_obj.to_string(), MagicVariableInstance {
                origin: self.origin,
                value: MagicVariableType::ObjectRef(obj_ref),
                range: self.range(&first_range)
            });
            obj_ref
        };
        for (obj_name, obj_range) in result.into_iter().rev() {
            if let Some(obj) = self.ctx.objects.get_mut(&store).unwrap().0.get_mut(&obj_name) {
                if let MagicVariableType::ObjectRef(id) = obj.get_common_type().unwrap_or(MagicVariableType::Unknown) {
                    store = id;
                } else {
                    let new_obj_id = self.ctx.create_obj();
                    obj.insert(MagicVariableInstance { 
                        origin: self.origin,
                        value: MagicVariableType::ObjectRef(new_obj_id), 
                        range: obj_range
                    });
                    store = new_obj_id;
                }
            } else {
                let new_obj_id = self.ctx.create_obj();
                self.ctx.objects.get_mut(&store).unwrap().insert(obj_name, MagicVariableInstance { 
                    origin: self.origin,
                    value: MagicVariableType::ObjectRef(new_obj_id), 
                    range: obj_range
                });
                store = new_obj_id;
            }
        }
        Some((store, self.get_string_from_accessor(&access.accessor)?.to_string()))
    }

    fn process_exp(&mut self, exp: &ASTExpression) -> MagicVariableType {
        match exp {
            ASTExpression::Binary(binary) => match &binary.left {
                ASTExpression::Identifier(ident) => {
                    let left_name = self.input.from_range(&ident.range).to_string();
                    let var_type = self.resolve_binary(&binary.operator, &binary.right);
                    self.ctx.variables.insert(
                        left_name,
                        MagicVariableInstance {
                            origin: self.origin,
                            value: var_type.clone(),
                            range: self.range(&ident.range),
                        },
                    );
                    var_type
                }
                ASTExpression::Access(access) => {
                    if let Some((obj_id, var_name)) = self.resolve_access(access) {
                        let right_type = self.resolve_binary(&binary.operator, &binary.right);
                        let range = self.range(access.accessor.range());
                        self.ctx.objects.get_mut(&obj_id).unwrap().insert(var_name, MagicVariableInstance {
                            origin: self.origin,
                            value: right_type.clone(),
                            range
                        });
                        right_type
                    } else {
                        MagicVariableType::Unknown
                    }
                }
                _ => {
                    exp.visit_each_child(self);
                    MagicVariableType::Unknown
                }
            },
            ASTExpression::String(_) => MagicVariableType::String,
            ASTExpression::Number(_) => MagicVariableType::Number,
            ASTExpression::Boolean(_) => MagicVariableType::Bool,
            ASTExpression::ArrayLit(_) => MagicVariableType::Array,
            ASTExpression::Identifier(ident) => {
                if let Some(typ) = self.ctx.variables.0.get_mut(self.input.from_range(&ident.range)) {
                    typ.get_common_type().unwrap_or(MagicVariableType::Unknown)
                } else {
                    MagicVariableType::Unknown
                }
            },
            ASTExpression::Access(acc) => {
                if let Some((store_id, name)) = self.resolve_access(acc) {
                    self.ctx.objects.get(&store_id).unwrap().0.get(&name).unwrap().get_common_type().unwrap_or(MagicVariableType::Unknown)
                } else {
                    MagicVariableType::Unknown
                }
            },
            ASTExpression::Call(call) => {
                if let ASTExpression::Access(access) = &call.expression {
                    if let Some("push" | "pop" | "join" | "slice" | "splice" | "shift" | "unshift") = self.get_string_from_accessor(&access.accessor) {
                        if let ASTExpression::Identifier(ident) = &access.expression {
                            let ident_text = self.input.from_range(&ident.range);
                            self.ctx.variables.insert(ident_text.to_string(), MagicVariableInstance {
                                origin: self.origin,
                                value: MagicVariableType::Array,
                                range: self.range(&ident.range)
                            });
                        } else if let ASTExpression::Access(access) = &access.expression {
                            if let Some((obj_id, name)) = self.resolve_access(access) {
                                let range = self.range(&access.range);
                                self.ctx.objects.get_mut(&obj_id).unwrap().insert(name, MagicVariableInstance {
                                    origin: self.origin,
                                    value: MagicVariableType::Array,
                                    range
                                });
                            }
                        }
                    }
                }
                MagicVariableType::Unknown
            },
            _ => {
                exp.visit_each_child(self);
                MagicVariableType::Unknown
            }
        }
    }

    pub fn run(mut self, exp: Vec<ASTExpression>) -> InputPresenter<'a> {
        exp.visit_each_child(&mut self);
        self.input
    }
}

impl<'a, ORIGIN: Eq + Hash + Default + Copy> Visitor for MagicVariableCollector<'a, ORIGIN> {
    fn expression(&mut self, exp: &ASTExpression) {
        self.process_exp(exp);
    }
}
