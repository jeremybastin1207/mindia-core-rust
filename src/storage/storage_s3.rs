use bytes::Bytes;
use std::error::Error;

use crate::adapter::s3::S3;
use crate::storage::storage_trait::Storage;

pub struct S3Storage {
    s3: S3,
}

impl S3Storage {
    pub fn new(s3: S3) -> Self {
        Self { s3 }
    }
}

impl Storage for S3Storage {
    fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn Error>> {
        println!("Uploading to S3");
        Ok(())
    }
}
