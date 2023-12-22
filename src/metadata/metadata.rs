use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub content_type: Option<String>,
    pub content_length: usize,
    pub embedded_metadata: HashMap<String, String>,
    pub derived_medias: Vec<Metadata>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            content_type: None,
            content_length: 0,
            embedded_metadata: HashMap::new(),
            derived_medias: Vec::new(),
            created_at: SystemTime::now(),
            updated_at: None,
        }
    }
}
