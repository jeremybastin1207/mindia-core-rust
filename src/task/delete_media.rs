use std::error::Error;
use std::sync::Arc;

use crate::media::Path;
use crate::metadata::MetadataStorage;
use crate::storage::FileStorage;

pub struct DeleteMedia {
    file_storage: Arc<dyn FileStorage>,
    cache_storage: Arc<dyn FileStorage>,
    metadata_storage: Arc<dyn MetadataStorage>,
}

impl DeleteMedia {
    pub fn new(
        file_storage: Arc<dyn FileStorage>,
        cache_storage: Arc<dyn FileStorage>,
        metadata_storage: Arc<dyn MetadataStorage>,
    ) -> DeleteMedia {
        DeleteMedia {
            file_storage: file_storage.clone(),
            cache_storage,
            metadata_storage,
        }
    }

    pub fn delete(&self, path: Path) -> Result<(), Box<dyn Error>> {
        let file_storage = self.file_storage.clone();
        let cache_storage = self.cache_storage.clone();
        let metadata_storage = self.metadata_storage.clone();

        file_storage.delete(path.as_str()?)?;
        cache_storage.delete(path.as_str()?)?;
        metadata_storage.delete(path.as_str()?)?;

        Ok(())
    }
}
