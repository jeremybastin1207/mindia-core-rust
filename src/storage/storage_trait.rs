use bytes::Bytes;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait FileStorage: Send + Sync {
    async fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn Error>>;
    async fn download(&self, path: &str) -> Result<Option<Bytes>, Box<dyn Error>>;
    async fn move_(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>>;
    async fn copy(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>>;
    async fn delete(&self, path: &str) -> Result<(), Box<dyn Error>>;
}
