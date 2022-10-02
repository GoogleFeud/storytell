use storytell_diagnostics::location::Range;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

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

#[derive(Debug, Default)]
/// Stores variables, and also their respective objects
/// and origins
pub struct VariableStore<O: Hash + Default + Eq> {
    pub variables: FxHashMap<u32, Variable<O>>,
    pub global: HashMap<String, u32>,
    pub objects: FxHashMap<u32, HashMap<String, u32>>,
    pub by_origin: FxHashMap<O, u32>,
    pub obj_counter: u32
}

impl<O: Hash + Default + Eq> VariableStore<O> {

    pub fn get_global(&self, key: &str) -> Option<&Variable<O>> {
        self.variables.get(self.global.get(key)?)
    }

    pub fn get_obj(&self, obj_id: &u32, key: &str) -> Option<&Variable<O>> {
        self.variables.get(self.objects.get(obj_id)?.get(key)?)
    }

    pub fn create_obj(&mut self) -> u32 {
        let object_id = self.obj_counter;
        self.obj_counter += 1;
        self.objects.insert(object_id, HashMap::new());
        object_id
    }

    pub fn insert_global(&mut self, origin: O, key: String, assignment: VariableAssignment<O>) {
        match self.global.entry(key) {
            Entry::Occupied(occupied) => {
                let item = occupied.get();
                self.variables.get_mut(item).unwrap().assignments.push(assignment);
                self.by_origin.insert(origin, *item);
            },
            Entry::Vacant(vacant) => {
                let mut entry = Variable::default();
                entry.assignments.push(assignment);
                let item_id = self.obj_counter;
                self.obj_counter += 1;
                self.variables.insert(item_id, entry);
                vacant.insert(item_id);
                self.by_origin.insert(origin, item_id);
            }
        }
    }

    pub fn insert_obj(&mut self, origin: O, key: String, obj_id: &u32, assignment: VariableAssignment<O>) {
        match self.objects.get_mut(obj_id).unwrap().entry(key) {
            Entry::Occupied(occupied) => {
                let item = occupied.get();
                self.variables.get_mut(item).unwrap().assignments.push(assignment);
                self.by_origin.insert(origin, *item);
            },
            Entry::Vacant(vacant) => {
                let mut entry = Variable::default();
                entry.assignments.push(assignment);
                let item_id = self.obj_counter;
                self.obj_counter += 1;
                self.variables.insert(item_id, entry);
                vacant.insert(item_id);
                self.by_origin.insert(origin, item_id);
            }
        }
    }

}