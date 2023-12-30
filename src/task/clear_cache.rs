use std::error::Error;
use std::sync::Arc;

use crate::metadata::MetadataStorage;
use crate::storage::FileStorage;

pub struct ClearCache {
    file_storage: Arc<dyn FileStorage>,
    cache_storage: Arc<dyn FileStorage>,
    metadata_storage: Arc<dyn MetadataStorage>,
}

impl ClearCache {
    pub fn new(
        file_storage: Arc<dyn FileStorage>,
        cache_storage: Arc<dyn FileStorage>,
        metadata_storage: Arc<dyn MetadataStorage>,
    ) -> Self {
        Self {
            file_storage,
            cache_storage,
            metadata_storage,
        }
    }

    pub fn clear(&self) -> Result<(), Box<dyn Error>> {
        let metadatas = self.metadata_storage.get_all().unwrap();

        for mut metadata in metadatas {
            let derived_medias = metadata.derived_medias.clone();

            for derived_media in derived_medias {
                self.cache_storage
                    .delete(derived_media.path.as_str().unwrap())?;

                metadata.remove_derived_media(&derived_media.path);
            }

            self.metadata_storage
                .save(metadata.path.as_str().unwrap(), metadata.clone())?;
        }

        Ok(())
    }
}
