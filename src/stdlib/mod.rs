pub mod modules;
pub mod primitives;
pub mod collections;

use crate::runtime::object::Object;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FieldsManager {
    pub fields: HashMap<String, Object>,
}

impl FieldsManager {
    pub fn make(fields: HashMap<String, Object>) -> Self {
        Self { fields }
    }

    pub fn access(&self, name: &str) -> Option<Object> {
        let field = self.fields.get(name);
        if field.is_none() {
            return None;
        }
        Some(field.unwrap().clone())
    }
}
