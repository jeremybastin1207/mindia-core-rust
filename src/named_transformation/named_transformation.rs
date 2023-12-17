use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::transform::Transformation;

pub type NamedTransformationMap = HashMap<String, NamedTransformation>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedTransformation {
    pub name: String,
    pub transformations: Vec<Transformation>,
}

impl NamedTransformation {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            transformations: vec![],
        }
    }
}
