use storytell_diagnostics::location::Range;
use std::{collections::{hash_map::Entry, HashMap}, hash::Hash};

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug)]
pub struct MagicVariableInstance<ORIGIN: Eq> {
    pub origin: ORIGIN,
    pub value: MagicVariableType,
    pub range: Range<usize>
}

#[derive(Default, Debug)]
pub struct MagicVariable<ORIGIN: Eq + Hash>(pub Vec<MagicVariableInstance<ORIGIN>>);

impl<ORIGIN: Eq + Hash> MagicVariable<ORIGIN> {

    pub fn get_common_type(&self) -> Option<MagicVariableType> {
        
        if self.0.is_empty() {
            None
        } else {
            let first = self.0[0].value.clone();
            for item in self.0.iter().skip(1) {
                if item.value != first {
                    return None;
                }
            }
            Some(first)
        }
    }

    pub fn has_common_type(&self) -> bool {
        if self.0.is_empty() {
            false
        } else {
            let first = &self.0[0].value;
            self.0.iter().skip(1).all(|v| &v.value == first)
        }
    }

    pub fn insert(&mut self, instance: MagicVariableInstance<ORIGIN>) {
        self.0.push(instance);
    }

}

// Need to find a way without Rc<RefCell>, but imagine a lot of restructuing would have to be made
#[derive(Debug, Default)]
pub struct MagicObject<ORIGIN: Eq + Hash + Default>(pub HashMap<String, MagicVariable<ORIGIN>>);

impl<ORIGIN: Eq + Hash + Default> MagicObject<ORIGIN> {

    pub fn insert(&mut self, key: String, var: MagicVariableInstance<ORIGIN>) {
        match self.0.entry(key) {
            Entry::Occupied(occupied) => {
                let item = occupied.into_mut();
                item.insert(var);
            },
            Entry::Vacant(vacant) => {
                let mut entry = MagicVariable::default();
                entry.insert(var);
                vacant.insert(entry);
            }
        }
    }

}