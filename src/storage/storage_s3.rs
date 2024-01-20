use bytes::Bytes;
use std::error::Error;
use async_trait::async_trait;

use crate::{
    adapter::s3::S3,
    storage::storage_trait::FileStorage,
};

pub struct S3Storage {
    s3: S3,
}

impl S3Storage {
    pub fn new(s3: S3) -> Self {
        Self { s3 }
    }
}

#[async_trait]
impl FileStorage for S3Storage {
    async fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn Error>> {
        self.s3.upload_object(path, data).await
    }

    async fn download(&self, path: &str) -> Result<Option<Bytes>, Box<dyn Error>> {
        let s3_object = self.s3.download_object(path).await?;
        Ok(Some(s3_object.body))

    }

    async fn move_(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
        self.s3.move_object(src, dst).await
    }

    async fn copy(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
        self.s3.copy_object(src, dst).await
    }

    async fn delete(&self, path: &str) -> Result<(), Box<dyn Error>> {
        self.s3.delete_object(path).await
    }
}
