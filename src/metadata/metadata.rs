use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::SystemTime;

pub type ContentLength = usize;
pub type EmbeddedMetadata = HashMap<String, String>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tag {
    value: String,
    confidence_score: f32,
    provider: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Metadata {
    base: BaseMetadata,
    embedded_metadata: EmbeddedMetadata,
    tags: Vec<Tag>,
    derived_medias: Vec<BaseMetadata>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseMetadata {
    content_type: Option<String>,
    content_length: ContentLength,
    created_at: Option<SystemTime>,
    updated_at: Option<SystemTime>,
}
