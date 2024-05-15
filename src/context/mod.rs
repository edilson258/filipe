use crate::runtime::object::{Object, ObjectInfo};
use crate::runtime::type_system::Type;
use crate::stdlib::modules::ModulesManager;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum ContextType {
    Global,
    Function,
    Loop,
    IfElse,
}

#[derive(Debug, Clone)]
pub struct Context {
    type_: ContextType,
    store: HashMap<String, ObjectInfo>,
    parent: Option<Rc<RefCell<Context>>>,
    pub modules: ModulesManager,
}

impl Context {
    pub fn make_from(parent: Rc<RefCell<Context>>, type_: ContextType) -> Self {
        Self {
            type_,
            store: HashMap::new(),
            parent: Some(parent),
            modules: ModulesManager::setup(),
        }
    }

    pub fn make_global(store: HashMap<String, ObjectInfo>) -> Self {
        Self {
            type_: ContextType::Global,
            store,
            parent: None,
            modules: ModulesManager::setup(),
        }
    }

    pub fn set(&mut self, name: String, type_: Type, value: Object, is_mut: bool) -> bool {
        if self.store.contains_key(&name) {
            return false;
        }
        self.store.insert(
            name,
            ObjectInfo {
                value,
                is_mut,
                type_,
            },
        );
        true
    }

    pub fn mutate(&mut self, name: String, value: Object) -> bool {
        if self.store.contains_key(&name) {
            let old = self.store.get_mut(&name).unwrap();
            if !old.is_mut {
                return false;
            }
            old.value = value;
            return true;
        }
        match self.parent {
            Some(ref p) => p.borrow_mut().mutate(name, value),
            None => false,
        }
    }

    pub fn has(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }

    pub fn resolve(&self, name: &str) -> Option<ObjectInfo> {
        if self.store.contains_key(name) {
            let obj = self.store.get(name).unwrap();
            return Some(obj.clone());
        }
        match self.parent {
            Some(ref p) => p.borrow().resolve(name),
            None => None,
        }
    }

    pub fn in_context_type(&self, ctx_type: ContextType) -> bool {
        if self.type_ == ctx_type {
            return true;
        }
        match self.parent {
            Some(ref p) => p.borrow().in_context_type(ctx_type),
            None => false,
        }
    }
}
