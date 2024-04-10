use super::object::Object;
use std::collections::HashMap;

pub struct Environment {
    store: HashMap<String, Object>,
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

    pub fn from(store: HashMap<String, Object>, parent: Option<Environment>) -> Self {
        let parent = match parent {
            Some(parent) => Some(Box::new(parent)),
            None => None,
        };
        Self { store, parent }
    }

    pub fn set_parent(&mut self, parent: Environment) {
        self.parent = Some(Box::new(parent));
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }

    pub fn resolve(&self, name: &str) -> Option<Object> {
        if self.store.contains_key(name) {
            let obj = self.store.get(name).unwrap().clone();
            return Some(obj);
        }
        return match &self.parent {
            Some(parent) => parent.resolve(name),
            None => None,
        };
    }
}
