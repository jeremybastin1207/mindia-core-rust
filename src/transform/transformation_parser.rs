use std::error::Error;

use super::{create_scaler, Transformation};
use crate::pipeline::PipelineStep;
use crate::task::UploadMediaContext;

#[derive(Default)]
pub struct TransformationParser;

impl TransformationParser {
    pub fn parse(
        &self,
        transformation: Transformation,
    ) -> Result<Box<dyn PipelineStep<UploadMediaContext>>, Box<dyn Error>> {
        match transformation.name.as_str() {
            "scale" => {
                let scaler = create_scaler(transformation)?;
                Ok(scaler as Box<dyn PipelineStep<UploadMediaContext>>)
            }
            _ => Err("Unknown transformation".into()),
        }
    }
}
