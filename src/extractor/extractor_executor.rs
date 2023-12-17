use std::error::Error;

use crate::extractor::{ContextExtractor, Extractor, ExtractorOutput};

pub struct ExtractorExecutor {}

impl ExtractorExecutor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract(
        &self,
        mut context: ContextExtractor,
        extractors: Vec<Box<dyn Extractor>>,
    ) -> Result<ExtractorOutput, Box<dyn Error>> {
        for extractor in extractors {
            context = extractor.extract(context)?;
        }

        Ok(context.output)
    }
}
