use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct RequiredFields {
    fields: HashMap<String, FieldValue>,
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
pub enum FieldValue {
    Number(i64),
    String(String),
}

impl RequiredFields {
    pub fn new() -> Self {
        RequiredFields {
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

impl Default for RequiredFields {
    fn default() -> Self {
        Self::new()
    }
}
