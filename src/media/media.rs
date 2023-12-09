use std::clone::Clone;
use std::fmt::Debug;
use std::path::PathBuf;

use crate::metadata::Metadata;

pub type Body = Vec<u8>;

#[derive(Debug, PartialEq, Clone)]
pub struct MediaHandler {
    path: PathBuf,
    body: Body,
    metadata: Metadata,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MediaLogicGroupHandler {
    media: MediaHandler,
    derived_medias: Vec<MediaHandler>,
}
