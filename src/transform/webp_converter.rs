use bytes::BytesMut;
use image::io::Reader as ImageReader;
use image::EncodableLayout;
use std::{error::Error, io::Cursor};
use webp::{Encoder, WebPMemory};

use crate::media::Path;

#[derive(Default)]
pub struct WebpConverter {}

impl WebpConverter {
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
