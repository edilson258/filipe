use std::collections::HashMap;

use crate::runtime::object::Object;

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub fields: HashMap<String, Object>,
}

impl Module {
    pub fn acc_field(&self, name: &str) -> Option<Object> {
        let object = self.fields.get(name);
        if object.is_none() {
            return None;
        }
        Some(object.unwrap().clone())
    }

    pub fn make(name: String, fields: HashMap<String, Object>) -> Self {
        Self { name, fields }
    }
}
