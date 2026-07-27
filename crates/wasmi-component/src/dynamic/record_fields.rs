use std::rc::Rc;

use crate::DynValue;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct RecordFields {
    pub fields: Rc<[(Rc<str>, DynValue)]>,
}

impl RecordFields {
    pub fn new(fields: Rc<[(Rc<str>, DynValue)]>) -> Self {
        Self { fields }
    }

    pub fn get_field(&self, name: &str) -> Option<&DynValue> {
        self.fields.iter().find_map(|(field_name, val)| {
            if field_name.as_ref() == name {
                Some(val)
            } else {
                None
            }
        })
    }
}
