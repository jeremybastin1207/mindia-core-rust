use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub name: String,
    pub key: String,
}
