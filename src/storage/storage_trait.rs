use bytes::Bytes;
use std::error::Error;

pub trait FileStorage: Send + Sync {
    fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn Error>>;
    fn download(&self, path: &str) -> Result<Option<Bytes>, Box<dyn Error>>;
    fn move_(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>>;
    fn copy(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>>;
    fn delete(&self, path: &str) -> Result<(), Box<dyn Error>>;
}
