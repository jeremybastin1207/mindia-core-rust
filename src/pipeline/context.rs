use std::collections::HashMap;

pub struct Context {
    values: Hashmap<String, String>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            values: Hashmap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }
}
