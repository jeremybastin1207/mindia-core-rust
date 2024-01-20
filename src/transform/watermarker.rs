// use bytes::{Bytes, BytesMut};
// use image::{EncodableLayout, GenericImage, GenericImageView, ImageBuffer, Rgba};
// use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
// use std::{error::Error, str::FromStr};
// use std::future::Future;
// use webp::{Encoder, WebPMemory};
// use crate::pipeline::PipelineContext;
//
// pub type OverlaySinkerFunc = Box<dyn Future<Output = Result<Bytes, Box<std::io::Error>>> + Send + Sync>;
//
// #[derive(Clone, Copy)]
// pub enum Anchor {
//     TopLeft,
//     TopCenter,
//     TopRight,
//     BottomLeft,
//     BottomCenter,
//     BottomRight,
//     LeftCenter,
//     RightCenter,
//     Center,
// }
//
// impl FromStr for Anchor {
//     type Err = std::io::Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_lowercase().as_str() {
//             "topcenter" | "centertop" => Ok(Anchor::TopCenter),
//             "topleft" | "lefttop" => Ok(Anchor::TopLeft),
//             "topright" | "righttop" => Ok(Anchor::TopRight),
//             "bottomcenter" | "centerbottom" => Ok(Anchor::BottomCenter),
//             "bottomleft" | "leftbottom" => Ok(Anchor::BottomLeft),
//             "bottomright" | "rightbottom" => Ok(Anchor::BottomRight),
//             "centerleft" | "leftcenter" => Ok(Anchor::LeftCenter),
//             "centerright" | "rightcenter" => Ok(Anchor::RightCenter),
//             "center" => Ok(Anchor::Center),
//             _ => Err(std::io::Error::new(
//                 std::io::ErrorKind::InvalidInput,
//                 "invalid anchor string",
//             )),
//         }
//     }
// }
//
// pub struct Watermarker {
//     anchor: Anchor,
//     padding: u32,
//     overlay_sinker: OverlaySinkerFunc,
// }
//
// impl Watermarker {
//     pub fn new(anchor: Anchor, padding: u32, overlay_sinker: OverlaySinkerFunc) -> Self {
//         Self {
//             anchor,
//             padding,
//             overlay_sinker,
//         }
//     }
//
//     pub async fn transform(&self, bytes: BytesMut) -> Result<BytesMut, Box<dyn Error>> {
//         let mut dst = BytesMut::new();
//
//         let mut img = image::load_from_memory(&bytes)?;
//
//         let overlay_bytes = self.overlay_sinker().await?;
//         let overlay = image::load_from_memory(&overlay_bytes)?;
//
//         let (img_w, img_h) = img.dimensions();
//         let (overlay_w, overlay_h) = overlay.dimensions();
//
//         let (x, y) = get_watermark_position(
//             (img_w, img_h),
//             (overlay_w, overlay_h),
//             self.anchor,
//             self.padding,
//         );
//
//         let mut overlay_buffer = ImageBuffer::new(img_w, img_h);
//         draw_filled_rect_mut(
//             &mut overlay_buffer,
//             Rect::at(0, 0).of_size(overlay_w, overlay_h),
//             Rgba([0, 0, 0, 0]),
//         );
//
//         overlay_buffer.copy_from(&overlay, x, y)?;
//
//         img.copy_from(&overlay_buffer, 0, 0)?;
//
//         let encoder: Encoder = Encoder::from_image(&img)?;
//         let encoded_webp: WebPMemory = encoder.encode(65f32);
//
//         dst.extend_from_slice(&encoded_webp.as_bytes());
//
//         Ok(dst)
//     }
// }
//
// fn get_watermark_position(
//     image_size: (u32, u32),
//     watermark_size: (u32, u32),
//     anchor: Anchor,
//     padding: u32,
// ) -> (u32, u32) {
//     let (img_w, img_h) = image_size;
//     let (wk_w, wk_h) = watermark_size;
//
//     let x = match anchor {
//         Anchor::TopLeft => padding as i32,
//         Anchor::TopCenter => (img_w / 2 - wk_w / 2) as i32,
//         Anchor::TopRight => (img_w - wk_w - padding) as i32,
//         Anchor::BottomLeft => padding as i32,
//         Anchor::BottomCenter => (img_w / 2 - wk_w / 2) as i32,
//         Anchor::BottomRight => (img_w - wk_w - padding) as i32,
//         Anchor::LeftCenter => padding as i32,
//         Anchor::RightCenter => (img_w - wk_w - padding) as i32,
//         Anchor::Center => (img_w / 2 - wk_w / 2) as i32,
//     };
//
//     let y = match anchor {
//         Anchor::TopLeft => padding as i32,
//         Anchor::TopCenter => padding as i32,
//         Anchor::TopRight => padding as i32,
//         Anchor::BottomLeft => (img_h - wk_h - padding) as i32,
//         Anchor::BottomCenter => (img_h - wk_h - padding) as i32,
//         Anchor::BottomRight => (img_h - wk_h - padding) as i32,
//         Anchor::LeftCenter => (img_h / 2 - wk_h / 2) as i32,
//         Anchor::RightCenter => (img_h / 2 - wk_h / 2) as i32,
//         Anchor::Center => (img_h / 2 - wk_h / 2) as i32,
//     };
//
//     (x as u32, y as u32)
// }
