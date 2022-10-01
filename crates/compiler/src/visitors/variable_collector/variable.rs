use storytell_diagnostics::location::Range;
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariableKind {
    String,
    Number,
    Bool,
    Array,
    ObjectRef(u32),
    Unknown
}

#[derive(Clone, Debug)]
/// Represents an assignment of a variable,
/// for example, x = 1, x += 33, x--, etc.
pub struct VariableAssignment<O: Hash> {
    pub origin: O,
    pub kind: VariableKind,
    pub range: Range<usize>
}

#[derive(Clone, Debug, Default)]
pub struct Variable<O: Hash> {
    pub assignments: Vec<VariableAssignment<O>>
}

impl<O: Hash> Variable<O> {

    pub fn get_common_kind(&self) -> Option<VariableKind> {
        let first_val = self.assignments.get(0)?.kind.clone();
        for item in self.assignments.iter().skip(1) {
            if item.kind != first_val {
                return None;
            }
        }
        Some(first_val)
    }

}

pub type ManagedVariable<O> = Arc<RefCell<Variable<O>>>;

#[derive(Debug, Default)]
/// Represents a javascript object
pub struct VariableObject<O: Hash>(pub HashMap<String, ManagedVariable<O>>);

impl<O: Hash + Default> VariableObject<O> {

    pub fn insert(&mut self, name: String, assignment: VariableAssignment<O>) -> ManagedVariable<O> {
        match self.0.entry(name) {
            Entry::Occupied(occupied) => {
                let item = occupied.get();
                item.borrow_mut().assignments.push(assignment);
                item.clone()
            },
            Entry::Vacant(vacant) => {
                let mut entry = Variable::default();
                entry.assignments.push(assignment);
                let managed_entry = Arc::from(RefCell::from(entry));
                vacant.insert(managed_entry.clone());
                managed_entry
            }
        }
    }

}


#[derive(Debug, Default)]
/// Stores variables, and also their respective objects
/// and origins
pub struct VariableStore<O: Hash + Default + Eq> {
    pub variables: VariableObject<O>,
    pub objects: FxHashMap<u32,  VariableObject<O>>,
    pub by_origin: FxHashMap<O, ManagedVariable<O>>,
    pub obj_counter: u32
}

impl<O: Hash + Default + Eq> VariableStore<O> {

    pub fn create_obj(&mut self) -> u32 {
        let id = self.obj_counter;
        self.obj_counter += 1;
        self.objects.insert(id, VariableObject::default());
        id
    }

}