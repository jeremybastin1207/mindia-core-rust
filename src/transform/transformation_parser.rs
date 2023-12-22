use std::error::Error;

use super::{create_scaler, Transformation, Transformer};

pub struct TransformationParser {}

impl TransformationParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(
        &self,
        transformations: Vec<Transformation>,
    ) -> Result<Vec<Box<dyn Transformer>>, Box<dyn Error>> {
        let mut transformers: Vec<Box<dyn Transformer>> = vec![];

        for transformation in transformations {
            match transformation.name.as_str() {
                "scale" => {
                    let scaler = create_scaler(transformation)?;
                    transformers.push(scaler as Box<dyn Transformer>);
                }
                _ => (),
            }
        }

        Ok(transformers)
    }
}
