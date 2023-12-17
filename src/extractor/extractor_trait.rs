use std::error::Error;

use crate::extractor::ContextExtractor;

pub trait Extractor {
    fn extract(&self, context: ContextExtractor) -> Result<ContextExtractor, Box<dyn Error>>;
}
