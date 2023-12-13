use bytes::Bytes;
use std::error::Error;

pub trait Extractor {
    type Output;

    fn extract(&self, data: Bytes) -> Result<Self::Output, Box<dyn Error>>;
}
