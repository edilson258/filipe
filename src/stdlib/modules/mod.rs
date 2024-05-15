mod io;
mod math;
mod random;

use super::FieldsManager;
use crate::runtime::object::Object;
use io::module_io;
use math::module_math;
use random::module_random;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub fields: FieldsManager,
}

impl Module {
    pub fn make(name: String, fields: HashMap<String, Object>) -> Self {
        Self {
            name,
            fields: FieldsManager::make(fields),
        }
    }
}

type ModInit = fn() -> Object;

#[derive(Debug, Clone)]
pub struct ModulesManager {
    modules: HashMap<String, ModInit>,
}

impl ModulesManager {
    pub fn setup() -> Self {
        let mut modules: HashMap<String, ModInit> = HashMap::new();
        modules.insert("Math".to_string(), module_math);
        modules.insert("IO".to_string(), module_io);
        modules.insert("random".to_string(), module_random);
        Self { modules }
    }

    pub fn access(&self, name: &str) -> Option<ModInit> {
        let mod_init = self.modules.get(name);
        if mod_init.is_none() {
            return None;
        }
        Some(mod_init.unwrap().clone())
    }
}
