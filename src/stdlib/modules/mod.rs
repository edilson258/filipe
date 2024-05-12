pub mod math;

use self::math::module_math;

use super::FieldsManager;
use crate::runtime::object::Object;
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
