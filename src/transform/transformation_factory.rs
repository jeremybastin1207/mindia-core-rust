use std::error::Error;

use super::{create_scaler, Transformation, TransformationDescriptionRegistry};
use crate::pipeline::PipelineStep;
use crate::task::UploadMediaContext;

pub struct TransformationFactory {
    transformation_description_registry: TransformationDescriptionRegistry,
}

impl TransformationFactory {
    pub fn new() -> TransformationFactory {
        TransformationFactory {
            transformation_description_registry: TransformationDescriptionRegistry::new(),
        }
    }

    pub fn build(
        &self,
        transformation: Transformation,
    ) -> Result<Box<dyn PipelineStep<UploadMediaContext>>, Box<dyn Error>> {
        self.transformation_description_registry
            .find_one(transformation.name.as_str())
            .ok_or("Unknown transformation")?;

        match transformation.name.as_str() {
            "scale" => {
                let scaler = create_scaler(transformation.clone())?;
                Ok(scaler as Box<dyn PipelineStep<UploadMediaContext>>)
            }
            _ => Err("Unknown transformation".into()),
        }
    }
}
