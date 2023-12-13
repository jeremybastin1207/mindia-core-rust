use bytes::BytesMut;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageOutputFormat, Rgba};
use std::error::Error;
use std::io::Cursor;

use crate::media::Path;
use crate::types::position::Position;
use crate::types::size::Size;

use crate::transform::Transform;

#[derive(PartialEq)]
pub enum CropStrategy {
    PadResizeCrop,
}

pub struct Scaler {
    size: Size,
    crop_strategy: CropStrategy,
    pad_color: Rgba<u8>,
}

impl Scaler {
    pub fn new(size: Size, crop_strategy: CropStrategy, pad_color: Rgba<u8>) -> Self {
        Self {
            size,
            crop_strategy,
            pad_color,
        }
    }
}

impl Transform for Scaler {
    fn transform(&self, path: &mut Path, mut bytes: BytesMut) -> Result<(), Box<dyn Error>> {
        /*         let img = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()?
            .decode()?;

        let img = img.resize_exact(
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

                    dst.write_to(&mut bytes, ImageOutputFormat::Webp)?;

                    return Ok(());
                }
            }
            _ => return Err("No valid crop strategy found".into()),
        } */

        Ok(())
    }
}
