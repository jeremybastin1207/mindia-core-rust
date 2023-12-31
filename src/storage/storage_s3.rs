use bytes::Bytes;
use std::error::Error;

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

impl FileStorage for S3Storage {
    fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn Error>> {
        let s3 = self.s3.clone();
        actix_rt::System::new().block_on(async move {
            s3.upload_object(path, data).await
        })
    }

    fn download(&self, path: &str) -> Result<Option<Bytes>, Box<dyn Error>> {
        let s3 = self.s3.clone();
        actix_rt::System::new().block_on(async move {
            let s3_object = s3.download_object(path).await?;
            Ok(Some(s3_object.body))
        })
    }

    fn move_(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
        let s3 = self.s3.clone();
        actix_rt::System::new().block_on(async move {
            s3.move_object(src, dst).await
        })
    }

    fn copy(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
        let s3 = self.s3.clone();
        actix_rt::System::new().block_on(async move {
            s3.copy_object(src, dst).await
        })
    }

    fn delete(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let s3 = self.s3.clone();
        actix_rt::System::new().block_on(async move {
            s3.delete_object(path).await
        })
    }
}
