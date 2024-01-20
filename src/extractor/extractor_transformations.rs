use std::error::Error;
use std::sync::Arc;

use crate::transform::{
    NamedTransformationStorage, TransformationDescriptor, TransformationDescriptorChain,
    TransformationTemplateRegistry,
};

const TRANSFORMATION_SEPARATOR: char = '/';
const TRANSFORMATION_NAME_SEPARATOR: char = ':';
const ARG_SEPARATOR: char = ',';
const VALUE_SEPARATOR: char = '_';
const NAMED_TRANSFORMATION_PREFIX: &str = "t_";

pub struct TransformationsExtractor {
    named_transformation_storage: Arc<dyn NamedTransformationStorage>,
    transformation_template_registry: Arc<TransformationTemplateRegistry>,
}

impl TransformationsExtractor {
    pub fn new(
        named_transformation_storage: Arc<dyn NamedTransformationStorage>,
        transformation_template_registry: Arc<TransformationTemplateRegistry>,
    ) -> Self {
        Self {
            named_transformation_storage,
            transformation_template_registry,
        }
    }

    pub fn extract_one(
        &self,
        transformation_chain_str: &str,
    ) -> Result<TransformationDescriptorChain, Box<dyn Error>> {
        let transformation_chains = self.extract(vec![transformation_chain_str])?;

        Ok(transformation_chains[0].clone())
    }

    pub fn extract(
        &self,
        transformation_chains_str: Vec<&str>,
    ) -> Result<Vec<TransformationDescriptorChain>, Box<dyn Error>> {
        let mut transformation_chains: Vec<TransformationDescriptorChain> = Vec::new();

        for transformation_chain_str in transformation_chains_str {
            let mut transformation_chain = TransformationDescriptorChain::default();

            let transformations_str = transformation_chain_str
                .split(TRANSFORMATION_SEPARATOR)
                .collect::<Vec<&str>>();

            for transformation_str in transformations_str {
                if transformation_str.starts_with(NAMED_TRANSFORMATION_PREFIX) {
                    let transformation_name = transformation_str
                        .strip_prefix(NAMED_TRANSFORMATION_PREFIX)
                        .unwrap();

                    let named_transformation = self
                        .named_transformation_storage
                        .get_by_name(transformation_name)?
                        .ok_or_else(|| {
                            std::io::Error::new(
                                std::io::ErrorKind::NotFound,
                                "Named transformation not found",
                            )
                        })?;

                    for transformation in named_transformation.transformations {
                        transformation_chain.add(transformation);
                    }
                } else {
                    let transformation_parts = transformation_str
                        .split(TRANSFORMATION_NAME_SEPARATOR)
                        .collect::<Vec<&str>>();

                    let transformation_name = transformation_parts[0].to_string();

                    let transformation_template = self
                        .transformation_template_registry
                        .find_one(transformation_name.as_str())
                        .ok_or("Unknown transformation")?;

                    let mut transformation = TransformationDescriptor::new(transformation_template);

                    if transformation_parts.len() > 1 {
                        let args = transformation_parts[1]
                            .split(ARG_SEPARATOR)
                            .collect::<Vec<&str>>();

                        for arg in args {
                            let arg_parts = arg.split(VALUE_SEPARATOR).collect::<Vec<&str>>();

                            let key = arg_parts[0].to_string();
                            let value = arg_parts[1].replace("%", "/").to_string();

                            transformation.add_arg(key, value);
                        }
                    }

                    transformation_chain.add(transformation);
                }
            }

            transformation_chains.push(transformation_chain);
        }

        Ok(transformation_chains)
    }
}
