use std::error::Error;
use std::sync::Arc;
use crate::handler::upload::PipelineStepFactory;
use crate::handler::UploadMediaContext;
use crate::pipeline::PipelineStep;
use crate::storage::FileStorage;
use crate::transform::{Colorizer, TransformationDescriptor};

pub struct ColorizerFactory {
    file_storage: Arc<dyn FileStorage>,
}

impl ColorizerFactory {
    pub fn new(
        file_storage: Arc<dyn FileStorage>,
    ) -> Self {
        Self {
            file_storage,
        }
    }
}

impl PipelineStepFactory for ColorizerFactory {
    fn create(
        &self,
        transformation_descriptor: TransformationDescriptor,
    ) -> Result<Box<dyn PipelineStep<UploadMediaContext>>, Box<dyn Error>> {
        let make_copy_of_original = transformation_descriptor
            .arg_values
            .get("c")
            .unwrap_or(&"false".to_string())
            .parse::<bool>()?;


        Ok(Box::new(Colorizer::new(
            self.file_storage.clone(),
        )))
    }
}