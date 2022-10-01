use std::hash::Hash;
use storytell_diagnostics::location::Range;
use storytell_js_parser::{input::InputPresenter, tokenizer::TokenKind, ast::{ASTExpression, ASTAccessContent, ASTAccess, Visitor, Visitable}};
use self::variable::{VariableStore, VariableKind, VariableAssignment};
pub mod variable;


pub struct VariableCollector<'a, O: Hash + Copy + Default + Eq> {
    pub current_origin: O,
    pub store: &'a mut VariableStore<O>,
    pub input: InputPresenter<'a>,
    pub start_pos: Range<usize>
}

impl<'a, O: Hash + Copy + Default + Eq> VariableCollector<'a, O> {

    fn range(&self, other: &Range<usize>) -> Range<usize> {
        let start = self.start_pos.start + other.start;
        Range {
            start,
            end: start + other.end,
        }
    }

    fn resolve_binary(&mut self, op: &TokenKind, right: &ASTExpression) -> VariableKind {
        match op {
            TokenKind::MinusEqualsOp | TokenKind::StarEqualsOp | TokenKind::StarEqualsOp
                if matches!(right, ASTExpression::Number(_)) => {
                VariableKind::Number
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
        let mut store = if let Some(var_type) = self.store.variables.0.get(first_obj) {
            if let VariableKind::ObjectRef(id) = var_type.borrow().get_common_kind().unwrap_or(VariableKind::Unknown) {
                id
            } else {
                return None;
                /*
                let obj_ref = self.store.create_obj();
                var_type.borrow_mut().assignments.push(VariableAssignment {
                    origin: self.current_origin,
                    kind: VariableKind::ObjectRef(obj_ref),
                    range: self.range(&first_range)
                });
                obj_ref
                */
            }
        } else {
            let obj_ref = self.store.create_obj();
            self.store.by_origin.insert(self.current_origin, self.store.variables.insert(first_obj.to_string(), VariableAssignment {
                origin: self.current_origin,
                kind: VariableKind::ObjectRef(obj_ref),
                range: self.range(&first_range)
            }));
            obj_ref
        };
        for (obj_name, obj_range) in result.into_iter().rev() {
            if let Some(obj) = self.store.objects.get_mut(&store).unwrap().0.get(&obj_name) {
                if let VariableKind::ObjectRef(id) = obj.borrow().get_common_kind().unwrap_or(VariableKind::Unknown) {
                    store = id;
                } else {
                    return None;
                    /*
                    let new_obj_id = self.store.create_obj();
                    obj.borrow_mut().assignments.push(VariableAssignment { 
                        origin: self.current_origin,
                        kind: VariableKind::ObjectRef(new_obj_id), 
                        range: obj_range
                    });
                    store = new_obj_id;
                    */
                }
            } else {
                let new_obj_id = self.store.create_obj();
                self.store.by_origin.insert(self.current_origin, self.store.objects.get_mut(&store).unwrap().insert(obj_name, VariableAssignment { 
                    origin: self.current_origin,
                    kind: VariableKind::ObjectRef(new_obj_id), 
                    range: obj_range
                }));
                store = new_obj_id;
            }
        }
        Some((store, self.get_string_from_accessor(&access.accessor)?.to_string()))
    }

    fn process_exp(&mut self, exp: &ASTExpression) -> VariableKind {
        match exp {
            ASTExpression::Binary(binary) => match &binary.left {
                ASTExpression::Identifier(ident) => {
                    let left_name = self.input.from_range(&ident.range).to_string();
                    let var_type = self.resolve_binary(&binary.operator, &binary.right);
                    self.store.by_origin.insert(self.current_origin, self.store.variables.insert(
                        left_name,
                        VariableAssignment {
                            origin: self.current_origin,
                            kind: var_type.clone(),
                            range: self.range(&ident.range),
                        },
                    ));
                    var_type
                }
                ASTExpression::Access(access) => {
                    if let Some((obj_id, var_name)) = self.resolve_access(access) {
                        let right_type = self.resolve_binary(&binary.operator, &binary.right);
                        let range = self.range(access.accessor.range());
                        self.store.by_origin.insert(self.current_origin, self.store.objects.get_mut(&obj_id).unwrap().insert(var_name, VariableAssignment {
                            origin: self.current_origin,
                            kind: right_type.clone(),
                            range
                        }));
                        right_type
                    } else {
                        VariableKind::Unknown
                    }
                }
                _ => {
                    exp.visit_each_child(self);
                    VariableKind::Unknown
                }
            },
            ASTExpression::String(_) => VariableKind::String,
            ASTExpression::Number(_) => VariableKind::Number,
            ASTExpression::Boolean(_) => VariableKind::Bool,
            ASTExpression::ArrayLit(_) => VariableKind::Array,
            ASTExpression::Identifier(ident) => {
                if let Some(typ) = self.store.variables.0.get_mut(self.input.from_range(&ident.range)) {
                    typ.borrow().get_common_kind().unwrap_or(VariableKind::Unknown)
                } else {
                    VariableKind::Unknown
                }
            },
            ASTExpression::Access(acc) => {
                if let Some((store_id, name)) = self.resolve_access(acc) {
                    self.store.objects.get(&store_id).unwrap().0.get(&name).unwrap().borrow().get_common_kind().unwrap_or(VariableKind::Unknown)
                } else {
                    VariableKind::Unknown
                }
            },
            ASTExpression::Call(call) => {
                if let ASTExpression::Access(access) = &call.expression {
                    if let Some("push" | "pop" | "join" | "slice" | "splice" | "shift" | "unshift") = self.get_string_from_accessor(&access.accessor) {
                        if let ASTExpression::Identifier(ident) = &access.expression {
                            let ident_text = self.input.from_range(&ident.range);
                            self.store.by_origin.insert(self.current_origin, self.store.variables.insert(ident_text.to_string(), VariableAssignment {
                                origin: self.current_origin,
                                kind: VariableKind::Array,
                                range: self.range(&ident.range)
                            }));
                        } else if let ASTExpression::Access(access) = &access.expression {
                            if let Some((obj_id, name)) = self.resolve_access(access) {
                                let range = self.range(&access.range);
                                self.store.by_origin.insert(self.current_origin, self.store.objects.get_mut(&obj_id).unwrap().insert(name, VariableAssignment {
                                    origin: self.current_origin,
                                    kind: VariableKind::Array,
                                    range
                                }));
                            }
                        }
                    }
                }
                VariableKind::Unknown
            },
            _ => {
                exp.visit_each_child(self);
                VariableKind::Unknown
            }
        }
    }

    pub fn run(mut self, exp: &Vec<ASTExpression>) -> InputPresenter<'a> {
        exp.visit_each_child(&mut self);
        self.input
    }
}

impl<'a, O: Hash + Default + Copy + Eq> Visitor for VariableCollector<'a, O> {
    fn expression(&mut self, exp: &ASTExpression) {
        self.process_exp(exp);
    }
}