use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    pub name: String,
    pub args: HashMap<String, String>,
}

impl Transformation {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            args: HashMap::new(),
        }
    }

    pub fn as_str(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.name);
        for (k, v) in &self.args {
            s.push_str(&format!(",{}-{}", k, v));
        }
        s
    }
}
