use bytes::BytesMut;
use image::{EncodableLayout, GenericImage, GenericImageView, ImageBuffer, Rgba};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use std::{error::Error, str::FromStr};
use std::sync::Arc;
use webp::{Encoder};
use crate::media::Path;
use crate::storage::FileStorage;
use crate::transform::{CropStrategy, Scaler};
use crate::transform::TransformationName::Scale;
use crate::types::Size;


pub struct Watermarker {
    anchor: Anchor,
    padding: u32,
    size: Size,
    overlay_path: Path,
    file_storage: Arc<dyn FileStorage>,
}

impl Watermarker {
    pub fn new(anchor: Anchor, padding: u32, size: Size, overlay_path: Path, file_storage: Arc<dyn FileStorage>) -> Self {
        Self {
            anchor,
            padding,
            size,
            overlay_path,
            file_storage,
        }
    }

    pub async fn transform(&self, bytes: BytesMut) -> Result<BytesMut, Box<dyn Error>> {
        match self.file_storage.download(self.overlay_path.as_str()).await? {
            Some(overlay_bytes) => {
                let mut img = image::load_from_memory(&bytes)?;
                let (img_w, img_h) = img.dimensions();

                let mut overlay_w = self.size.width;
                let mut overlay_h = self.size.height;
                let aspect_ratio = overlay_w as f32 / overlay_h as f32;

                if !watermark_can_fit_in_image((img_w, img_h), (overlay_w, overlay_h), self.padding) {
                     match find_watermark_size_that_fit_image(img.dimensions(), self.padding, aspect_ratio) {
                        Ok((w, h)) => {
                            overlay_w = w;
                            overlay_h = h;
                         },
                        Err(e) => return Err(e.into()),
                    }
                }

                let mut overlay_bytes_mut = BytesMut::new();
                overlay_bytes_mut.extend_from_slice(&overlay_bytes);

                let scaler = Scaler::new(Size::new(overlay_w, overlay_h), CropStrategy::ForcedCrop, Rgba([0, 0, 0, 0]));
                let scaled_overlay = scaler.transform(self.overlay_path.clone(), overlay_bytes_mut)?;

                let mut overlay = image::load_from_memory(&scaled_overlay)?;

                let (x, y) = get_watermark_position(
                    (img_w, img_h),
                    (overlay_w, overlay_h),
                    self.anchor,
                    self.padding,
                );

                let mut overlay_buffer = ImageBuffer::new(overlay_w, overlay_h);
                draw_filled_rect_mut(
                    &mut overlay_buffer,
                    Rect::at(0, 0).of_size(overlay_w, overlay_h),
                    Rgba([0, 0, 0, 0]),
                );

                overlay_buffer.copy_from(&overlay, 0, 0)?;

                img.copy_from(&overlay_buffer, x, y)?;

                let encoder = Encoder::from_image(&img)?;
                let encoded_webp = encoder.encode(65f32);

                let mut dst = BytesMut::new();
                dst.extend_from_slice(&encoded_webp.as_bytes());

                Ok(dst)
            },
            None => Ok(bytes)
        }
    }
}

fn watermark_can_fit_in_image(image_size: (u32, u32), watermark_size: (u32, u32), padding: u32) -> bool {
    let (img_w, img_h) = image_size;
    let (wk_w, wk_h) = watermark_size;

    img_w > wk_w + padding * 2 && img_h > wk_h + padding * 2
}

fn find_watermark_size_that_fit_image(image_size: (u32, u32), padding: u32, watermark_aspect_ratio: f32) -> Result<(u32, u32), Box<dyn Error>> {
    let (img_w, img_h) = image_size;

    let mut width = img_w - (padding * 2);
    let mut height = (width as f32 / watermark_aspect_ratio).round() as u32;

    if width <= 0 || height <= 0 || height > img_h - (padding * 2) {
        height = img_h - (padding * 2);
        width = (height as f32 * watermark_aspect_ratio).round() as u32;

        if width <= 0 || height <= 0 {
            return Err("Watermark is too big".into());
        }
    }

    Ok((width, height))
}

fn get_watermark_position(
    image_size: (u32, u32),
    watermark_size: (u32, u32),
    anchor: Anchor,
    padding: u32,
) -> (u32, u32) {
    let (img_w, img_h) = image_size;
    let (wk_w, wk_h) = watermark_size;

    let x = match anchor {
        Anchor::TopLeft => padding as i32,
        Anchor::TopCenter => (img_w / 2 - wk_w / 2) as i32,
        Anchor::TopRight => (img_w - wk_w - padding) as i32,
        Anchor::BottomLeft => padding as i32,
        Anchor::BottomCenter => (img_w / 2 - wk_w / 2) as i32,
        Anchor::BottomRight => (img_w - wk_w - padding) as i32,
        Anchor::LeftCenter => padding as i32,
        Anchor::RightCenter => (img_w - wk_w - padding) as i32,
        Anchor::Center => (img_w / 2 - wk_w / 2) as i32,
    };

    let y = match anchor {
        Anchor::TopLeft => padding as i32,
        Anchor::TopCenter => padding as i32,
        Anchor::TopRight => padding as i32,
        Anchor::BottomLeft => (img_h - wk_h - padding) as i32,
        Anchor::BottomCenter => (img_h - wk_h - padding) as i32,
        Anchor::BottomRight => (img_h - wk_h - padding) as i32,
        Anchor::LeftCenter => (img_h / 2 - wk_h / 2) as i32,
        Anchor::RightCenter => (img_h / 2 - wk_h / 2) as i32,
        Anchor::Center => (img_h / 2 - wk_h / 2) as i32,
    };

    (x as u32, y as u32)
}

#[derive(Clone, Copy)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    LeftCenter,
    RightCenter,
    Center,
}

impl FromStr for Anchor {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "topcenter" | "centertop" => Ok(Anchor::TopCenter),
            "topleft" | "lefttop" => Ok(Anchor::TopLeft),
            "topright" | "righttop" => Ok(Anchor::TopRight),
            "bottomcenter" | "centerbottom" => Ok(Anchor::BottomCenter),
            "bottomleft" | "leftbottom" => Ok(Anchor::BottomLeft),
            "bottomright" | "rightbottom" => Ok(Anchor::BottomRight),
            "centerleft" | "leftcenter" => Ok(Anchor::LeftCenter),
            "centerright" | "rightcenter" => Ok(Anchor::RightCenter),
            "center" => Ok(Anchor::Center),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid anchor string",
            )),
        }
    }
}
