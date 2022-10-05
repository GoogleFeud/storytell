use std::{hash::Hash, collections::{HashMap, hash_map::Entry}};
use rustc_hash::FxHashMap;
use storytell_diagnostics::location::Range;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VariableKind {
    String,
    Number,
    Bool,
    Array,
    Object,
    Map,
    Ref(String),
    Unknown
}

#[derive(Debug)]
/// Represents an assignment of a variable,
/// for example, x = 1, x += 33, x--, etc.
pub struct VariableAssignment {
    pub kind: VariableKind,
    pub range: Range<usize>
}

#[derive(Debug, Default)]
/// A variable which has multiple possible types,
/// which span across multiple different origins
/// (an origin can be a file, a string, etc.)
pub struct Variable<O: Hash + Eq> {
    pub assignments: FxHashMap<O, Vec<VariableAssignment>>
}

impl<O: Hash + Eq> Variable<O> {

    pub fn get_common_kind(&self) -> Option<VariableKind> {
        let mut iter = self.assignments.values().flatten();
        let first_val = iter.next()?.kind.clone();
        for item in iter {
            if item.kind != first_val {
                return None;
            }
        }
        return Some(first_val);
    }

}

#[derive(Debug, Default)]
pub struct VariableContainer<O: Hash + Eq>(pub HashMap<String, Variable<O>>);

impl<O: Hash + Eq + Clone> VariableContainer<O> {
    pub fn remove(&mut self, origin: &O) {
        for variable in self.0.values_mut() {
            variable.assignments.remove(origin);
        }
    }

    pub fn insert(&mut self, origin: O, assignments: AssignmentStore) {
        for (variable_name, assignments) in assignments.0.into_iter() {
            match self.0.entry(variable_name) {
                Entry::Occupied(mut occupied) => {
                    let val = occupied.get_mut();
                    val.assignments.insert(origin.clone(), assignments);
                },
                Entry::Vacant(vacant) => {
                    let mut hash_map = FxHashMap::default();
                    hash_map.insert(origin.clone(), assignments);
                    vacant.insert(Variable {
                        assignments: hash_map
                    });
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct AssignmentStore(pub HashMap<String, Vec<VariableAssignment>>);

impl AssignmentStore {

    pub fn insert(&mut self, key: String, value: VariableAssignment) {
        match self.0.entry(key) {
            Entry::Occupied(mut occupied) => {
                let val = occupied.get_mut();
                val.push(value);
            },
            Entry::Vacant(vacant) => {
                vacant.insert(vec![value]);
            }
        }
    }

    pub fn append(&mut self, store: AssignmentStore) {
        for (key, mut value) in store.0.into_iter() {
            match self.0.entry(key) {
                Entry::Occupied(mut occupied) => {
                    let val = occupied.get_mut();
                    val.append(&mut value);
                },
                Entry::Vacant(vacant) => {
                    vacant.insert(value);
                }
            }
        }
    }

}