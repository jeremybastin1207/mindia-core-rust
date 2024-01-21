use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use crate::handler::upload::pipeline_step_factory_trait::PipelineStepFactory;
use crate::handler::upload::UploadMediaContext;
use crate::media::Path;
use crate::pipeline::PipelineStep;
use crate::storage::FileStorage;
use crate::transform::{TransformationDescriptor, Watermarker};
use crate::transform::watermarker::Anchor;
use crate::types::Size;

pub struct WatermarkerFactory {
    pub file_storage: Arc<dyn FileStorage>,
}

impl WatermarkerFactory {
    pub fn new(file_storage: Arc<dyn FileStorage>) -> Self {
        Self { file_storage }
    }
}

impl PipelineStepFactory for WatermarkerFactory {
    fn create(
        &self,
        transformation_descriptor: TransformationDescriptor,
    ) -> Result<Box<dyn PipelineStep<UploadMediaContext>>, Box<dyn Error>> {
        let path = transformation_descriptor
            .arg_values
            .get("f")
            .ok_or("Key 'path' not found")?
            .parse::<String>()
            .map_err(|_| "Failed to parse 't' value to String")?;

        let padding = transformation_descriptor
            .arg_values
            .get("p")
            .ok_or("Key 'padding' not found")?
            .parse::<u32>()
            .map_err(|_| "Failed to parse 'p' value to u32")?;

        let anchor = transformation_descriptor
            .arg_values
            .get("a")
            .ok_or("Key 'a' not found")?
            .parse::<String>()
            .map_err(|_| "Failed to parse 'a' value to String")?;

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

        let path = Path::new(path.as_str())?;
        let anchor = Anchor::from_str(&anchor)?;
        let file_storage = Arc::clone(&self.file_storage);

        let size = Size::new(width, height);


        Ok(Box::new(Watermarker::new(anchor, padding, size, path, file_storage)))
    }
}
