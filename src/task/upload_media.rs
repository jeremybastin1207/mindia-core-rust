use std::error::Error;
use std::sync::Arc;

use bytes::BytesMut;

use crate::storage::Storage;

pub struct UploadMedia {
    storage: Arc<dyn Storage>,
}

impl UploadMedia {
    pub fn new(storage: Arc<dyn Storage>) -> UploadMedia {
        UploadMedia { storage }
    }

    pub fn upload(&self, picture_data: BytesMut) -> Result<(), Box<dyn Error>> {
        let byte_slice: &[u8] = picture_data.as_ref();
        self.storage.upload(byte_slice)
    }
}
