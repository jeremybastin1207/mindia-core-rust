use bytes::Bytes;
use std::error::Error;

pub trait Storage {
    fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn Error>>;
}
