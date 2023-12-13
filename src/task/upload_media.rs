use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use crate::extractor::{ExifExtractor, Extractor};
use crate::media::Path;
use crate::metadata::{Metadata, MetadataStorage};
use crate::storage::FileStorage;
use crate::transform::{Transform, WebpConverter};

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

    pub fn upload(&self, mut path: Path, body: BytesMut) -> Result<(), Box<dyn Error>> {
        let mut metadata = Metadata::new();
        metadata.embedded_metadata = ExifExtractor::new().extract(body.clone().freeze())?;

        WebpConverter::new().transform(&mut path, body.clone())?;

        let path_str = path.as_str()?;
        {
            let file_storage = self.file_storage.lock().unwrap();
            file_storage.upload(path_str, body.into())?;
        }
        {
            let mut metadata_storage = self.metadata_storage.lock().unwrap();
            metadata_storage.save(path_str, metadata)?;
        }

        Ok(())
    }
}
