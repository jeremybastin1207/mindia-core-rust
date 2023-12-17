use image::io::Reader as ImageReader;
use std::error::Error;
use std::io::Cursor;

use crate::transform::Transform;

use super::ContextTransform;

pub struct WebpConverter {}

impl WebpConverter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transform for WebpConverter {
    fn transform(&self, mut context: ContextTransform) -> Result<ContextTransform, Box<dyn Error>> {
        let img = ImageReader::new(Cursor::new(&context.body))
            .with_guessed_format()?
            .decode()?;

        context.body.clear();
        context.body.extend_from_slice(img.as_bytes());

        context.path.set_extension("webp");

        Ok(context)
    }
}
