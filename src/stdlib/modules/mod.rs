pub mod math;

use crate::runtime::object::Object;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub fields: HashMap<String, Object>,
}

impl Module {
    pub fn make(name: String, fields: HashMap<String, Object>) -> Self {
        Self { name, fields }
    }

    pub fn acc_field(&self, name: &str) -> Option<Object> {
        let object = self.fields.get(name);
        if object.is_none() {
            return None;
        }
        Some(object.unwrap().clone())
    }
}
