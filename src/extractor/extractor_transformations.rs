use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::named_transformation::NamedTransformationStorage;
use crate::transform::Transformation;

const TRANSFORMATION_SEPARATOR: char = '/';
const TRANSFORMATION_NAME_SEPARATOR: char = ':';
const ARG_SEPARATOR: char = ',';
const VALUE_SEPARATOR: char = '_';
const NAMED_TRANSFORMATION_PREFIX: &str = "t_";

pub struct TransformationsExtractor {
    named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
}

impl TransformationsExtractor {
    pub fn new(named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>) -> Self {
        Self {
            named_transformation_storage,
        }
    }

    pub fn extract(
        &self,
        transformations_str: &str,
    ) -> Result<Vec<Transformation>, Box<dyn Error>> {
        let mut transformations: Vec<Transformation> = Vec::new();

        let transformations_str = transformations_str
            .split(TRANSFORMATION_SEPARATOR)
            .collect::<Vec<&str>>();

        for transformation_str in transformations_str {
            if transformation_str.starts_with(NAMED_TRANSFORMATION_PREFIX) {
                let transformation_name = transformation_str
                    .strip_prefix(NAMED_TRANSFORMATION_PREFIX)
                    .unwrap();

                let named_transformation = self
                    .named_transformation_storage
                    .lock()
                    .unwrap()
                    .get_by_name(transformation_name)?
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "Named transformation not found",
                        )
                    })?;

                for tranformation in named_transformation.transformations {
                    transformations.push(tranformation);
                }
            } else {
                let mut transformation = Transformation::new();

                let transformation_parts = transformation_str
                    .split(TRANSFORMATION_NAME_SEPARATOR)
                    .collect::<Vec<&str>>();

                transformation.name = transformation_parts[0].to_string();

                if transformation_parts.len() > 1 {
                    let args = transformation_parts[1]
                        .split(ARG_SEPARATOR)
                        .collect::<Vec<&str>>();

                    for arg in args {
                        let arg_parts = arg.split(VALUE_SEPARATOR).collect::<Vec<&str>>();

                        let key = arg_parts[0].to_string();
                        let value = arg_parts[1].to_string();

                        transformation.args.insert(key, value);
                    }
                }

                transformations.push(transformation);
            }
        }

        Ok(transformations)
    }
}
