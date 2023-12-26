use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::media::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub path: Path,
    pub content_type: Option<String>,
    pub content_length: usize,
    pub embedded_metadata: HashMap<String, String>,
    pub derived_medias: Vec<Metadata>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            path: Path::default(),
            content_type: None,
            content_length: 0,
            embedded_metadata: HashMap::new(),
            derived_medias: Vec::new(),
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}
