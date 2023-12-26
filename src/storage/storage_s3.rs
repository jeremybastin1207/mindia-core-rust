use bytes::Bytes;
use std::error::Error;

use crate::adapter::s3::S3;
use crate::storage::storage_trait::FileStorage;

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
        println!("Uploading to S3");
        Ok(())
    }

    fn download(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
        println!("Downloading from S3");
        Ok(Bytes::new())
    }
}
