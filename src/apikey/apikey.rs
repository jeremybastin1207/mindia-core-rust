use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub name: String,
    pub key: String,
}

pub type ApiKeyMap = HashMap<String, ApiKey>;
