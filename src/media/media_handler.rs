use bytes::Bytes;
use std::clone::Clone;
use std::fmt::Debug;

use crate::media::Path;
use crate::metadata::Metadata;

#[derive(Debug, Clone)]
pub struct MediaHandler {
    path: Path,
    body: Bytes,
    metadata: Metadata,
}

impl MediaHandler {
    pub fn new(path: Path, body: Bytes, metadata: Metadata) -> Self {
        Self {
            path,
            body,
            metadata,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn body(&self) -> &Bytes {
        &self.body
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
