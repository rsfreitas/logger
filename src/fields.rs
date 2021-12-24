
use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct RequiredFields {
    fields: HashMap<String, FieldValue>,
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
pub enum FieldValue {
    Number(i32),
    String(String),
}

impl RequiredFields {
    pub fn new() -> Self {
        RequiredFields{
            fields: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, value: FieldValue) -> &mut Self {
        self.fields.insert(key.to_string(), value);
        self
    }

    pub fn data(&self) -> HashMap<String, FieldValue> {
        self.fields.clone()
    }
}

#[derive(Serialize)]
pub struct Field {
    pub name: String,
    pub value: String,
}

