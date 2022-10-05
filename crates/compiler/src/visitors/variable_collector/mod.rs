use storytell_diagnostics::location::Range;
use storytell_js_parser::{input::InputPresenter, ast::{Visitor, ASTExpression, Visitable, ASTAccess, ASTAccessContent}, tokenizer::TokenKind};
use self::variable::{AssignmentStore, VariableKind, VariableAssignment};

pub mod variable;

pub struct VariableCollector<'a> {
    pub collected: AssignmentStore,
    pub input: InputPresenter<'a>,
    pub start_pos: Range<usize>
}

impl<'a> VariableCollector<'a> {

    pub fn run(input: InputPresenter<'a>, start_pos: Range<usize>, ast: &Vec<ASTExpression>) -> (AssignmentStore, InputPresenter<'a>) {
        let mut collector = VariableCollector {
            collected: AssignmentStore::default(),
            input,
            start_pos
        };
        ast.visit_each_child(&mut collector);
        (collector.collected, collector.input)
    }

    fn range(&self, other: &Range<usize>) -> Range<usize> {
        let start = self.start_pos.start + other.start;
        Range {
            start,
            end: start + other.end,
        }
    }

    fn resolve_binary(&mut self, op: &TokenKind, right: &ASTExpression) -> VariableKind {
        match op {
            TokenKind::PlusEqualsOp => {
                match right {
                    ASTExpression::String(_) | ASTExpression::StringTemplate(_) => VariableKind::String,
                    ASTExpression::Number(_) => VariableKind::Number,
                    _ => VariableKind::Unknown
                }
            },
            TokenKind::MinusEqualsOp | TokenKind::StarEqualsOp | TokenKind::SlashEqualsOp => {
                if matches!(right, ASTExpression::Number(_)) {
                    VariableKind::Number
                } else {
                    VariableKind::Unknown
                }
            }
            _ => self.process_exp(right),
        }
    }

    fn get_string_from_accessor(&self, accessor: &ASTAccessContent) -> Option<&str> {
        match accessor {
            ASTAccessContent::Identifier(ident) => Some(self.input.from_range(&ident.range)),
            _ => None
        }
    }

    fn resolve_access(&mut self, access: &ASTAccess) -> Option<String> {
        let mut collected: Vec<String> = vec![];
        collected.push(self.get_string_from_accessor(&access.accessor)?.to_string());
        let mut left = &access.expression;
        while let ASTExpression::Access(acc) = left {
            collected.push(self.get_string_from_accessor(&acc.accessor)?.to_string());
            left = &acc.expression;
        };
        if let ASTExpression::Identifier(ident) = left {
            collected.push(self.input.from_range(&ident.range).to_string())
        } else {
            return None;
        }
        collected.reverse();
        Some(collected.join("."))
    }

    fn add_variable(&mut self, name: &ASTExpression, kind: VariableKind) -> Option<VariableKind> {
        match name {
            ASTExpression::Identifier(ident) => {
                let left_name = self.input.from_range(&ident.range).to_string();
                self.collected.insert(left_name.clone(), VariableAssignment {
                    kind: kind.clone(),
                    range: self.range(&ident.range)
                });
                if matches!(kind, VariableKind::Array | VariableKind::Map | VariableKind::Object) {
                    Some(VariableKind::Ref(left_name))
                } else {
                    Some(kind)
                }
            }
            ASTExpression::Access(access) => {
                if let Some(name) = self.resolve_access(access) {
                    let range = self.range(access.accessor.range());
                    self.collected.insert(name.clone(), VariableAssignment {
                        kind: kind.clone(),
                        range
                    });
                    if matches!(kind, VariableKind::Array | VariableKind::Map | VariableKind::Object) {
                        Some(VariableKind::Ref(name))
                    } else {
                        Some(kind)
                    }
                } else {
                    None
                }
            }
            _ => None
        }
    }

    fn process_exp(&mut self, exp: &ASTExpression) -> VariableKind {
        match exp {
            ASTExpression::Binary(binary) => {
                let kind = self.resolve_binary(&binary.operator, &binary.right);
                if let Some(kind) = self.add_variable(&binary.left, kind) {
                    kind
                } else {
                    binary.visit_each_child(self);
                    VariableKind::Unknown
                }
            },
            ASTExpression::String(_) => VariableKind::String,
            ASTExpression::Number(_) => VariableKind::Number,
            ASTExpression::Boolean(_) => VariableKind::Bool,
            ASTExpression::ArrayLit(_) => VariableKind::Array,
            ASTExpression::Identifier(ident) => {
                VariableKind::Ref(self.input.from_range(&ident.range).to_string())
            },
            ASTExpression::Access(acc) => {
                if let Some(name) = self.resolve_access(acc) {
                    VariableKind::Ref(name)
                } else {
                    VariableKind::Unknown
                }
            },
            ASTExpression::Call(call) => {
                if let ASTExpression::Access(access) = &call.expression {
                    match self.get_string_from_accessor(&access.accessor) {
                        Some("push" | "pop" | "join" | "slice" | "splice" | "shift" | "unshift") => { self.add_variable(&access.expression, VariableKind::Array); },
                        Some("set" | "clear" | "has" | "get" | "keys" | "values") => { self.add_variable(&access.expression, VariableKind::Map); },
                        _ => {}
                    };
                }
                VariableKind::Unknown
            },
            _ => {
                exp.visit_each_child(self);
                VariableKind::Unknown
            }
        }
    }


}

impl<'a> Visitor for VariableCollector<'a> {
    fn expression(&mut self, exp: &ASTExpression) {
        self.process_exp(exp);
    }
}