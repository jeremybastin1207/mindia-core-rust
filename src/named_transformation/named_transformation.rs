use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedTransformation {
    pub name: String,
    pub transformations: String,
}

pub type NamedTransformationMap = HashMap<String, NamedTransformation>;
