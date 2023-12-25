use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{TransformationName, TransformationTemplate};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformationDescriptor {
    pub transformation_template: TransformationTemplate,
    pub arg_values: HashMap<String, String>,
}

impl TransformationDescriptor {
    pub fn new(transformation_template: TransformationTemplate) -> Self {
        Self {
            transformation_template,
            arg_values: HashMap::new(),
        }
    }

    pub fn name(&self) -> &TransformationName {
        &self.transformation_template.name
    }

    pub fn add_arg(&mut self, arg_name: String, arg_value: String) {
        self.arg_values.insert(arg_name, arg_value);
    }

    pub fn as_str(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.transformation_template.name.as_str());
        for (k, v) in &self.arg_values {
            s.push_str(&format!(",{}-{}", k, v));
        }
        s
    }
}
