use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;

use crate::extractor::ExifExtractor;
use crate::media::{MediaHandler, Path};
use crate::metadata::{Metadata, MetadataStorage};
use crate::storage::FileStorage;

pub struct UploadMedia {
    file_storage: Arc<Mutex<dyn FileStorage>>,
    metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
}

impl UploadMedia {
    pub fn new(
        file_storage: Arc<Mutex<dyn FileStorage>>,
        metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
    ) -> UploadMedia {
        UploadMedia {
            file_storage,
            metadata_storage,
        }
    }

    pub fn upload(&self, path: Path, body: BytesMut) -> Result<(), Box<dyn Error>> {
        let exif_data = ExifExtractor::new().extract(body.clone())?;

        let metadata = Metadata {
            content_type: None,
            content_length: 0,
            embedded_metadata: exif_data,
            derived_medias: Vec::new(),
            created_at: SystemTime::now(),
            updated_at: Some(SystemTime::now()),
        };

        let media_handler = MediaHandler::new(path, body.clone().into(), metadata.clone());

        let path_str = media_handler.path().as_str()?;

        let mut file_storage = self.file_storage.lock().unwrap();
        file_storage.upload(path_str, body.into())?;

        let mut metadata_storage = self.metadata_storage.lock().unwrap();
        metadata_storage.save(path_str, metadata)?;

        Ok(())
    }
}
