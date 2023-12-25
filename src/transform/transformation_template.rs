use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::TransformationName;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationArg {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationTemplate {
    pub name: TransformationName,
    pub description: String,
    pub args: HashMap<String, TransformationArg>,
}

impl TransformationTemplate {
    pub fn new() -> Self {
        Self {
            name: TransformationName::Unset,
            description: String::new(),
            args: HashMap::new(),
        }
    }

    pub fn with_name(self, name: TransformationName) -> Self {
        Self { name, ..self }
    }

    pub fn with_description(self, description: String) -> Self {
        Self {
            description,
            ..self
        }
    }

    pub fn with_arg(mut self, arg_name: String, arg_description: String) -> Self {
        self.args.insert(
            arg_name.clone(),
            TransformationArg {
                name: arg_name.clone(),
                description: arg_description,
            },
        );
        self
    }
}
