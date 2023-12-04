/* use std::error::Error;
use std::sync::Arc;

use crate::storage::FileStorage;

pub struct UploadMedia {
    file_storage: Arc<dyn FileStorage>,
}

impl UploadMedia {
    pub fn new(file_storage: Arc<dyn FileStorage>) -> UploadMedia {
        UploadMedia { file_storage }
    }

    pub fn upload(&self, picture_data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.file_storage.upload(picture_data)
    }
}
 */