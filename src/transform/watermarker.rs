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
    width: u32,
    height: u32,
    overlay_path: Path,
    file_storage: Arc<dyn FileStorage>,
}

impl Watermarker {
    pub fn new(anchor: Anchor, padding: u32, width: u32, height: u32, overlay_path: Path, file_storage: Arc<dyn FileStorage>) -> Self {
        Self {
            anchor,
            padding,
            width,
            height,
            overlay_path,
            file_storage,
        }
    }

    pub async fn transform(&self, bytes: BytesMut) -> Result<BytesMut, Box<dyn Error>> {
        match self.file_storage.download(self.overlay_path.as_str()).await? {
            Some(overlay_bytes) => {
                let mut img = image::load_from_memory(&bytes)?;

                let mut overlay_bytes_mut = BytesMut::new();
                overlay_bytes_mut.extend_from_slice(&overlay_bytes);

                let scaler = Scaler::new(Size::new(self.width, self.height), CropStrategy::ForcedCrop, Rgba([0, 0, 0, 0]));
                let scaled_overlay = scaler.transform(self.overlay_path.clone(), overlay_bytes_mut)?;

                let mut overlay = image::load_from_memory(&scaled_overlay)?;

                let (img_w, img_h) = img.dimensions();
                let (overlay_w, overlay_h) = overlay.dimensions();

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
