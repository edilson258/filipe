pub mod math;

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
