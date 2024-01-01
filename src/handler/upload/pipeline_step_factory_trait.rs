use std::error::Error;

use super::UploadMediaContext;
use crate::pipeline::PipelineStep;
use crate::transform::TransformationDescriptor;

pub trait PipelineStepFactory {
    fn create(
        &self,
        transformation_descriptor: TransformationDescriptor,
    ) -> Result<Box<dyn PipelineStep<UploadMediaContext>>, Box<dyn Error>>;
}
