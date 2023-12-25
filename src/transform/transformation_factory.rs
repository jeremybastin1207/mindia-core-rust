use std::error::Error;

use super::{create_scaler, TransformationDescriptorChain, TransformationName};
use crate::pipeline::PipelineStep;
use crate::task::UploadMediaContext;

#[derive(Default)]
pub struct TransformationFactory;

impl TransformationFactory {
    pub fn build(
        &self,
        transformation_descriptor_chain: TransformationDescriptorChain,
    ) -> Result<Vec<Box<dyn PipelineStep<UploadMediaContext>>>, Box<dyn Error>> {
        let mut transformation_descriptors: Vec<Box<dyn PipelineStep<UploadMediaContext>>> = vec![];

        for transformation_descriptor in transformation_descriptor_chain.iter() {
            match transformation_descriptor.name() {
                TransformationName::Scale => {
                    let scaler = create_scaler(transformation_descriptor.clone())?;
                    transformation_descriptors
                        .push(scaler as Box<dyn PipelineStep<UploadMediaContext>>);
                }
                _ => return Err("Unknown transformation".into()),
            }
        }

        Ok(transformation_descriptors)
    }
}
