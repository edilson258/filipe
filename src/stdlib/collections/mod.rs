use crate::runtime::object::Object;
use core::fmt;
use std::collections::HashMap;

use super::FieldsManager;

#[derive(Clone, Debug)]
pub struct Array {
    pub inner: Vec<Object>,
    pub fields: FieldsManager,
}

impl Array {
    pub fn from(init: Vec<Object>) -> Self {
        Self {
            fields: FieldsManager::make(Self::setup_fields(init.len() as i64)),
            inner: init,
        }
    }

    pub fn make_empty() -> Self {
        Self {
            inner: vec![],
            fields: FieldsManager::make(Self::setup_fields(0)),
        }
    }

    fn setup_fields(len: i64) -> HashMap<String, Object> {
        let mut fields: HashMap<String, Object> = HashMap::new();
        fields.insert("length".to_string(), Object::Int(len));
        fields
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (index, x) in self.inner.iter().enumerate() {
            if self.inner.len() - 1 == index {
                write!(f, "{}", x)?;
            } else {
                write!(f, "{}, ", x)?;
            }
        }
        write!(f, "]")
    }
}
