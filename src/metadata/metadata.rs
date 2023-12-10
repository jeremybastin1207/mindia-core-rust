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
