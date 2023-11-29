use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;

pub type Body = Vec<u8>;
pub type ContentLength = usize;
pub type Metadata = HashMap<String, String>;

#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
    value: String,
    confidence_score: f32,
    provider: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Media {
    path: Path,
    body: Body,
    content_type: Option<String>,
    content_length: ContentLength,
    embedded_metadata: Metadata,
    tags: Vec<Tag>,
    derived_medias: Vec<DerivedMedia>,
    created_at: Option<SystemTime>,
    updated_at: Option<SystemTime>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DerivedMedia {
    path: Path,
    body: Body,
    content_type: Option<String>,
    content_length: ContentLength,
    created_at: Option<SystemTime>,
    updated_at: Option<SystemTime>,
}