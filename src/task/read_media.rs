use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::media::Path;
use crate::metadata::{Metadata, MetadataStorage};

pub struct ReadMedia {
    metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
}

impl ReadMedia {
    pub fn new(metadata_storage: Arc<Mutex<dyn MetadataStorage>>) -> Self {
        Self {
            metadata_storage: metadata_storage,
        }
    }

    pub fn read(&self, path: Path) -> Result<Option<Metadata>, Box<dyn Error>> {
        let result = self
            .metadata_storage
            .lock()
            .unwrap()
            .get_by_path(path.as_str()?)?;

        Ok(result)
    }
}
