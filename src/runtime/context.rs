use super::{
    object::{Object, ObjectInfo},
    type_system::Type,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Context {
    store: HashMap<String, ObjectInfo>,
    parent: Option<Rc<RefCell<Context>>>,
}

impl Context {
    pub fn empty(parent: Option<Rc<RefCell<Context>>>) -> Self {
        Self {
            store: HashMap::new(),
            parent,
        }
    }

    pub fn from(store: HashMap<String, ObjectInfo>, parent: Option<Context>) -> Self {
        let parent = match parent {
            Some(parent) => Some(Rc::new(RefCell::new(parent))),
            None => None,
        };
        Self { store, parent }
    }

    pub fn add_entry(
        &mut self,
        name: String,
        value: Object,
        type_: Type,
        is_assignable: bool,
    ) -> bool {
        if self.store.contains_key(&name) {
            return false;
        }
        self.store.insert(
            name,
            ObjectInfo {
                is_assignable,
                type_,
                value,
            },
        );
        true
    }

    pub fn update_entry(&mut self, name: &str, value: Object) -> bool {
        let old_entry = match self.store.get_mut(name) {
            Some(object_info) => object_info,
            None => {
                return match self.parent {
                    Some(ref parent) => parent.borrow_mut().update_entry(name, value),
                    None => false,
                }
            }
        };
        if !old_entry.is_assignable {
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
            Some(ref parent) => parent.borrow().resolve(name),
            None => None,
        };
    }

    pub fn is_declared(&self, name: &str) -> bool {
        if self.store.contains_key(name) {
            return true;
        }
        return match &self.parent {
            Some(ref parent) => parent.borrow().is_declared(name),
            None => false,
        };
    }
}
