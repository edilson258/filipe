use super::FieldsManager;
use crate::runtime::object::Object;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Primitive<T> {
    pub value: T,
    pub fields: FieldsManager,
}

impl<T> Primitive<T> {
    pub fn make(value: T, fields: HashMap<String, Object>) -> Primitive<T> {
        let primitive = Primitive {
            value,
            fields: FieldsManager::make(fields),
        };
        primitive
    }
}

pub fn make_string(value: String) -> Primitive<String> {
    let mut str_fields: HashMap<String, Object> = HashMap::new();
    str_fields.insert("length".to_string(), Object::Int(value.len() as i64));
    Primitive::<String>::make(value, str_fields)
}
