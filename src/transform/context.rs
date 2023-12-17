use bytes::BytesMut;

use crate::media::Path;

#[derive(Debug, Clone)]
pub struct ContextTransform {
    pub path: Path,
    pub body: BytesMut,
}

impl ContextTransform {
    pub fn new(path: Path, body: BytesMut) -> Self {
        Self { path, body }
    }
}
