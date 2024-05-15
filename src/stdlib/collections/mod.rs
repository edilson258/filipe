use crate::runtime::{
    object::{BuiltInFuncReturnValue, Object, ObjectInfo},
    runtime_error::{ErrorKind, RuntimeError},
};
use core::fmt;
use std::collections::HashMap;

use super::{primitives::make_integer, FieldsManager};

#[derive(Clone, Debug)]
pub struct Array {
    pub inner: Vec<Object>,
    pub fields: FieldsManager,
}

impl Array {
    pub fn from(init: Vec<Object>) -> Self {
        Self {
            fields: FieldsManager::make(Self::setup_fields()),
            inner: init,
        }
    }

    pub fn make_empty() -> Self {
        Self {
            inner: vec![],
            fields: FieldsManager::make(Self::setup_fields()),
        }
    }

    fn setup_fields() -> HashMap<String, Object> {
        let mut fields: HashMap<String, Object> = HashMap::new();
        fields.insert("length".to_string(), Object::BuiltInFunction(array_length));
        fields
    }
}

fn array_length(args: Vec<ObjectInfo>) -> BuiltInFuncReturnValue {
    if args.len() != 1 {
        return BuiltInFuncReturnValue::Error(RuntimeError {
            kind: ErrorKind::ArgumentError,
            msg: format!("method length takes 0 args but provided {}", args.len() - 1),
        });
    }

    let len = match &args[0].value {
        Object::Array {
            inner,
            items_type: _,
        } => inner.inner.len(),
        _ => {
            return BuiltInFuncReturnValue::Error(RuntimeError {
                kind: ErrorKind::ArgumentError,
                msg: format!("method length accept arrays only"),
            });
        }
    };

    BuiltInFuncReturnValue::Object(Object::Int(make_integer(len as i64)))
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
