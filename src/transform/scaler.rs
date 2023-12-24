use bytes::BytesMut;
use image::io::Reader as ImageReader;
use image::EncodableLayout;
use image::{GenericImageView, ImageBuffer, Rgba};
use std::error::Error;
use std::io::Cursor;
use webp::{Encoder, WebPMemory};

use crate::media::Path;
use crate::types::position::Position;
use crate::types::size::Size;

#[derive(PartialEq)]
pub enum CropStrategy {
    PadResizeCrop,
    ForcedCrop,
}

pub struct Scaler {
    size: Size,
    crop_strategy: CropStrategy,
    pad_color: Rgba<u8>,
}

impl Scaler {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            crop_strategy: CropStrategy::ForcedCrop,
            pad_color: Rgba([0, 0, 0, 0]),
        }
    }

    pub fn with_strategy(self, crop_strategy: CropStrategy) -> Self {
        Self {
            crop_strategy,
            ..self
        }
    }

    pub fn with_pad_color(self, pad_color: Rgba<u8>) -> Self {
        Self { pad_color, ..self }
    }

    pub fn transform(
        &self,
        mut path: Path,
        mut body: BytesMut,
    ) -> Result<BytesMut, Box<dyn Error>> {
        let img = ImageReader::new(Cursor::new(&body))
            .with_guessed_format()?
            .decode()?;

        let img = img.resize(
            self.size.width,
            self.size.height,
            image::imageops::FilterType::Lanczos3,
        );

        let (img_w, img_h) = img.dimensions();

        match self.crop_strategy {
            CropStrategy::PadResizeCrop => {
                if img_w != self.size.width || img_h != self.size.height {
                    let mut dst =
                        ImageBuffer::from_pixel(self.size.width, self.size.height, self.pad_color);
                    let pos = match img.width() < self.size.width {
                        true => Position {
                            x: (self.size.width / 2) - (img.width() / 2),
                            y: 0,
                        },
                        false => Position {
                            x: 0,
                            y: (self.size.height / 2) - (img.height() / 2),
                        },
                    };

                    image::imageops::overlay(&mut dst, &img, pos.x.into(), pos.y.into());
                }
            }
            CropStrategy::ForcedCrop => {}
            _ => return Err("No valid crop strategy found".into()),
        }

        let encoder: Encoder = Encoder::from_image(&img)?;
        let encoded_webp: WebPMemory = encoder.encode(65f32);

        body.clear();
        body.extend_from_slice(&encoded_webp.as_bytes());

        path.set_extension("webp");

        Ok(body)
    }
}
