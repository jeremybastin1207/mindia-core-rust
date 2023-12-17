use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::extractor::extractor_trait::Extractor;
use crate::extractor::ContextExtractor;
use crate::named_transformation::NamedTransformationStorage;
use crate::transform::Transformation;

const TRANSFORMATION_SEPARATOR: char = '/';
const TRANSFORMATION_NAME_SEPARATOR: char = ':';
const ARG_SEPARATOR: char = ',';
const VALUE_SEPARATOR: char = '_';
const NAMED_TRANSFORMATION_PREFIX: &str = "t_";

pub struct NamedTransformationExtractor {
    named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
}

impl NamedTransformationExtractor {
    pub fn new(named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>) -> Self {
        Self {
            named_transformation_storage,
        }
    }
}

impl Extractor for NamedTransformationExtractor {
    fn extract(&self, mut context: ContextExtractor) -> Result<ContextExtractor, Box<dyn Error>> {
        let transformations_str = context
            .transformations_str
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
                    context.output.transformations.push(tranformation);
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

                context.output.transformations.push(transformation);
            }
        }

        Ok(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::Path;
    use crate::named_transformation::{MockNamedTransformationStorage, NamedTransformation};

    #[test]
    fn test_extract_named_transformation() {
        let mut mock_storage = MockNamedTransformationStorage::new();

        mock_storage
            .expect_get_by_name()
            .with(mockall::predicate::eq("test_name"))
            .returning(|_| Ok(Some(NamedTransformation::new())));

        let extractor = NamedTransformationExtractor::new(Arc::new(Mutex::new(mock_storage)));

        let context = ContextExtractor::new(
            "t_test_name".to_string(),
            Path::new("test_file".to_string()).unwrap(),
            "test_file".into(),
        );

        let result = extractor.extract(context);

        assert_eq!(result.unwrap().output.transformations.len(), 1);
        assert_eq!(result.unwrap().output.transformations[0].name, "test_name");
    }

    #[test]
    fn test_extract_transformations() {}
}
