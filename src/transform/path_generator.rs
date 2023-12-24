use std::error::Error;

use super::Transformation;
use crate::media::Path;

#[derive(Default)]
pub struct PathGenerator;

impl PathGenerator {
    pub fn transform(
        &self,
        path: &Path,
        transformations: &Vec<Transformation>,
    ) -> Result<Path, Box<dyn Error>> {
        let mut path = Path::generate(path.as_str()?)?;

        if !transformations.is_empty() {
            let suffix = transformations
                .iter()
                .map(|transformation| transformation.as_str())
                .collect::<Vec<_>>()
                .join(",");

            path = path.derive_with_suffix(&suffix)?;
        }

        Ok(path)
    }
}
