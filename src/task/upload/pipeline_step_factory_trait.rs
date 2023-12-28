use std::error::Error;

use crate::pipeline::PipelineStep;
use crate::task::UploadMediaContext;
use crate::transform::TransformationDescriptor;

pub trait PipelineStepFactory {
    fn create(
        &self,
        transformation_descriptor: TransformationDescriptor,
    ) -> Result<Box<dyn PipelineStep<UploadMediaContext>>, Box<dyn Error>>;
}
