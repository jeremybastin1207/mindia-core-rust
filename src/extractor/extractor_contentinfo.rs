use std::error::Error;
use bytes::BytesMut;
use crate::metadata::Metadata;

#[derive(Default)]
pub struct ContentInfoExtractor {}

impl ContentInfoExtractor {
    pub fn extract(
        &self,
        mut metadata: Metadata,
        body: BytesMut,
    ) -> Result<Metadata, Box<dyn Error>> {
        let content_type = match metadata.path.extension() {
           "webp" => "image/webp",
            _ => "image/octet-stream",
        };

        metadata.content_type = Some(content_type.to_string());
        metadata.content_length = body.len();

        Ok(metadata)
    }
}
