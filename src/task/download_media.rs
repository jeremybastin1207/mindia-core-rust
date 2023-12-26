use bytes::Bytes;
use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use crate::extractor::ExifExtractor;
use crate::media::MediaGroupHandle;
use crate::media::Path;
use crate::metadata::MetadataStorage;
use crate::pipeline::{Pipeline, PipelineExecutor, Sinker, Source};
use crate::storage::FileStorage;
use crate::transform::TransformationDescriptorChain;
use crate::transform::{PathGenerator, TransformationFactory, WebpConverter};

use super::UploadMediaContext;

pub struct DownloadMedia {
    file_storage: Arc<Mutex<dyn FileStorage>>,
    cache_storage: Arc<Mutex<dyn FileStorage>>,
    metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
}

impl DownloadMedia {
    pub fn new(
        file_storage: Arc<Mutex<dyn FileStorage>>,
        cache_storage: Arc<Mutex<dyn FileStorage>>,
        metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
    ) -> DownloadMedia {
        DownloadMedia {
            file_storage,
            cache_storage,
            metadata_storage,
        }
    }
    pub fn download(&self, path: Path) -> Result<Option<Bytes>, Box<dyn Error>> {
        let bytes = self.file_storage.lock().unwrap().download(path.as_str()?)?;

        Ok(Some(bytes))
    }
}
