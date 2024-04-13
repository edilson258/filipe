use super::object::{self, Object};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ObjectInfo {
    pub is_assinable: bool,
    pub value: Object
}

#[derive(Debug, Clone)]
pub struct Environment{
    store: HashMap<String, ObjectInfo>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn empty(parent: Option<Environment>) -> Self {
        let parent = match parent {
            Some(parent) => Some(Box::new(parent)),
            None => None,
        };

        Self {
            store: HashMap::new(),
            parent,
        }
    }

    pub fn from(store: HashMap<String, ObjectInfo>, parent: Option<Environment>) -> Self {
        let parent = match parent {
            Some(parent) => Some(Box::new(parent)),
            None => None,
        };
        Self { store, parent }
    }

    pub fn add_entry(&mut self, name: String, value: Object, is_assinable: bool) -> bool {
        if self.store.contains_key(&name) {
            return false;
        }
        self.store.insert(name, ObjectInfo {
            is_assinable,
            value,
        });
        true
    }

    pub fn update_entry(&mut self, name: &str, value: Object) -> bool {
        let old_entry = match self.store.get_mut(name) {
            Some(object_info) => object_info,
            None => return false,
        };
        if !old_entry.is_assinable {
            return false;
        }
        old_entry.value = value;
        true
    }

    pub fn resolve(&self, name: &str) -> Option<ObjectInfo> {
        if self.store.contains_key(name) {
            let obj = self.store.get(name).unwrap().clone();
            return Some(obj);
        }
        return match &self.parent {
            Some(parent) => parent.resolve(name),
            None => None,
        };
    }

    pub fn is_declared(&self, name: &str) -> bool {
        if self.store.contains_key(name) {
            return true;
        }
        return match &self.parent {
            Some(parent) => parent.is_declared(name),
            None => false
        };
    }
}
