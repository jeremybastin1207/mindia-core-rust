use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;

use crate::storage::Storage;

pub struct UploadMedia {
    storage: Arc<dyn Storage>,
}

impl UploadMedia {
    pub fn new(storage: Arc<dyn Storage>) -> UploadMedia {
        UploadMedia { storage }
    }

    pub fn upload(&self, picture_data: BytesMut) -> Result<(), Box<dyn Error>> {
        self.storage.upload("test/test.jpeg", picture_data.into())
    }
}
