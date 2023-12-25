use std::error::Error;

use super::TransformationDescriptorChain;
use crate::media::Path;

#[derive(Default)]
pub struct PathGenerator;

impl PathGenerator {
    pub fn transform(
        &self,
        path: &Path,
        transformation_descriptor_chain: TransformationDescriptorChain,
    ) -> Result<Path, Box<dyn Error>> {
        let mut path = Path::generate(path.as_str()?)?;

        if !transformation_descriptor_chain.is_empty() {
            let suffix = transformation_descriptor_chain
                .iter()
                .map(|transformation_descriptor| transformation_descriptor.as_str())
                .collect::<Vec<_>>()
                .join(",");

            path = path.derive_with_suffix(&suffix)?;
        }

        Ok(path)
    }
}
