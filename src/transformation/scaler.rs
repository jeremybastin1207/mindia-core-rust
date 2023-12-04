use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

use position::Position;
use size::Size;

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

    pub fn execute(&self, img: DynamicImage) -> Result<DynamicImage, Box<dyn std::error::Error>> {
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
                    return Ok(DynamicImage::ImageRgba8(dst));
                }
            }
            _ => return Err("No valid crop strategy found".into()),
        }

        Ok(img)
    }
}
