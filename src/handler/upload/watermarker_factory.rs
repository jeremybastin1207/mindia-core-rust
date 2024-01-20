// use std::{error::Error, str::FromStr, sync::Arc};
//
// use super::{PipelineStepFactory, UploadMediaContext};
// use crate::{
//     media::Path,
//     pipeline::PipelineStep,
//     storage::FileStorage,
//     transform::{ TransformationDescriptor},
// };
//
// pub struct WatermarkerFactory {
//     pub file_storage: Arc<dyn FileStorage>,
// }
//
// impl WatermarkerFactory {
//     pub fn new(file_storage: Arc<dyn FileStorage>) -> Self {
//         Self { file_storage }
//     }
// }
//
// impl PipelineStepFactory for WatermarkerFactory {
//     fn create(
//         &self,
//         transformation_descriptor: TransformationDescriptor,
//     ) -> Result<Box<dyn PipelineStep<UploadMediaContext>>, Box<dyn Error>> {
//         let path = transformation_descriptor
//             .arg_values
//             .get("t")
//             .ok_or("Key 'path' not found")?
//             .parse::<String>()
//             .map_err(|_| "Failed to parse 't' value to String")?;
//
//         let padding = transformation_descriptor
//             .arg_values
//             .get("p")
//             .ok_or("Key 'padding' not found")?
//             .parse::<u32>()
//             .map_err(|_| "Failed to parse 'p' value to u32")?;
//
//         let anchor = transformation_descriptor
//             .arg_values
//             .get("a")
//             .ok_or("Key 'a' not found")?
//             .parse::<String>()
//             .map_err(|_| "Failed to parse 'a' value to String")?;
//
//         let path = Path::new(path.as_str())?;
//         let anchor = Anchor::from_str(&anchor)?;
//         let file_storage = Arc::clone(&self.file_storage);
//
//         let overlay_sinker: OverlaySinkerFunc = Box::new(async move {
//             // return err directly:
//             Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "Failed to download file",
//             )))
//
//             // let result = file_storage.download(path.as_str()).await.map_err(|e| Box::new(e) as Box<dyn Error>);
//             // match result {
//             //     Ok(Some(bytes)) => Ok(bytes),
//             //     Ok(None) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download file"))),
//             //     Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download file"))),
//             // }
//         });
//
//         Ok(Box::new(Watermarker::new(anchor, padding, overlay_sinker)))
//     }
// }
