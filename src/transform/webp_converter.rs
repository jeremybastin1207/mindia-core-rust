use bytes::BytesMut;
use image::io::Reader as ImageReader;
use std::error::Error;
use std::io::Cursor;

use crate::media::Path;
use crate::transform::Transform;

pub struct WebpConverter {}

impl WebpConverter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transform for WebpConverter {
    fn transform(&self, path: &mut Path, mut bytes: BytesMut) -> Result<(), Box<dyn Error>> {
        let img = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()?
            .decode()?;

        bytes.clear();
        bytes.extend_from_slice(img.as_bytes());

        path.set_extension("webp");

        Ok(())
    }
}
