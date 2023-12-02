use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use std::time::SystemTime;

pub type Body = Vec<u8>;
pub type ContentLength = usize;
pub type Metadata = HashMap<String, String>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tag {
    value: String,
    confidence_score: f32,
    provider: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Media {
    path: PathBuf,
    body: Body,
    content_type: Option<String>,
    content_length: ContentLength,
    embedded_metadata: Metadata,
    tags: Vec<Tag>,
    derived_medias: Vec<DerivedMedia>,
    created_at: Option<SystemTime>,
    updated_at: Option<SystemTime>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivedMedia {
    path: PathBuf,
    body: Body,
    content_type: Option<String>,
    content_length: ContentLength,
    created_at: Option<SystemTime>,
    updated_at: Option<SystemTime>,
}
