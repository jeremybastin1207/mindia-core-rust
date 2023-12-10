use bytes::Bytes;
use std::error::Error;

pub trait FileStorage: Send + Sync {
    fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn Error>>;
}
