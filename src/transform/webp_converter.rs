use bytes::BytesMut;
use image::io::Reader as ImageReader;
use image::EncodableLayout;
use webp::{Encoder, WebPMemory};

use std::error::Error;
use std::io::Cursor;

use crate::media::Path;

pub struct WebpConverter {}

impl WebpConverter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(
        &self,
        mut path: Path,
        mut body: BytesMut,
    ) -> Result<(Path, BytesMut), Box<dyn Error>> {
        let img = ImageReader::new(Cursor::new(&body))
            .with_guessed_format()?
            .decode()?;

        let encoder: Encoder = Encoder::from_image(&img)?;

        let encoded_webp: WebPMemory = encoder.encode(65f32);

        body.clear();
        body.extend_from_slice(&encoded_webp.as_bytes());

        path.set_extension("webp");

        Ok((path, body))
    }
}
