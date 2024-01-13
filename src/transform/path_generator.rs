use std::error::Error;
use aws_sdk_s3::primitives::event_stream::HeaderValue::Uuid;

use super::TransformationDescriptorChain;
use crate::media::Path;

#[derive(Default)]
pub struct PathGenerator;

impl PathGenerator {
    pub fn transform(
        &self,
        mut path: Path,
        transformation_descriptor_chain: TransformationDescriptorChain,
    ) -> Result<Path, Box<dyn Error>> {
        if !transformation_descriptor_chain.is_empty() {
            let suffix = transformation_descriptor_chain
                .iter()
                .map(|transformation_descriptor| transformation_descriptor.as_str())
                .collect::<Vec<_>>()
                .join(",");

            path = path.add_suffix_to_filename(&suffix)?;
        }

        Ok(path)
    }
}
