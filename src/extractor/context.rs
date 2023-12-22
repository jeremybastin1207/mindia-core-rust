use bytes::Bytes;

use crate::media::Path;
use crate::metadata::Metadata;
use crate::transform::Transformation;

#[derive(Debug, Clone)]
pub struct ExtractorOutput {
    pub metadata: Metadata,
    pub transformations: Vec<Transformation>,
}

#[derive(Debug, Clone)]
pub struct ContextExtractor {
    pub transformations_str: String,
    pub path: Path,
    pub body: Bytes,
    pub output: ExtractorOutput,
}

impl ContextExtractor {
    pub fn new(transformations_str: String, path: Path, body: Bytes) -> Self {
        Self {
            transformations_str,
            path,
            body,
            output: ExtractorOutput {
                metadata: Metadata::default(),
                transformations: vec![],
            },
        }
    }
}
