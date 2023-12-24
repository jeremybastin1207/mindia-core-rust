use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationDescription {
    pub name: String,
    pub description: String,
    pub args: HashMap<String, String>,
}

impl TransformationDescription {
    pub fn with_name(self, name: String) -> Self {
        Self { name, ..self }
    }

    pub fn with_description(self, description: String) -> Self {
        Self {
            description,
            ..self
        }
    }

    pub fn with_arg(mut self, arg_name: String, arg_description: String) -> Self {
        self.args.insert(arg_name, arg_description);
        self
    }
}
