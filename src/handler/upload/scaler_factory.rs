use std::error::Error;

use image::Rgba;

use super::{PipelineStepFactory, UploadMediaContext};
use crate::pipeline::PipelineStep;
use crate::transform::{CropStrategy, Scaler, TransformationDescriptor};
use crate::types::Size;

#[derive(Default)]
pub struct ScalerFactory;

impl PipelineStepFactory for ScalerFactory {
    fn create(
        &self,
        transformation_descriptor: TransformationDescriptor,
    ) -> Result<Box<dyn PipelineStep<UploadMediaContext>>, Box<dyn Error>> {
        let height = transformation_descriptor
            .arg_values
            .get("h")
            .ok_or("Key 'h' not found")?
            .parse::<u32>()
            .map_err(|_| "Failed to parse 'h' value to u32")?;

        let width = transformation_descriptor
            .arg_values
            .get("w")
            .ok_or("Key 'w' not found")?
            .parse::<u32>()
            .map_err(|_| "Failed to parse 'w' value to u32")?;

        Ok(Box::new(Scaler::new(
            Size::new(width, height),
            CropStrategy::ForcedCrop,
            Rgba([0, 0, 0, 0]),
        )))
    }
}
