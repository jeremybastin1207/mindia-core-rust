use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

use crate::metadata::MetadataStorage;
use crate::scheduler::{Details, Task, TaskExecutor, TaskStatus};
use crate::storage::FileStorage;

const METADATA_LIMIT: u32 = 100;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClearCacheTaskDetails {
    pub before_created_at: DateTime<Utc>,
}

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
}

impl TaskExecutor for ClearCache {
    fn run(&self, mut task: Task) -> Result<Task, Box<dyn Error>> {
        println!("Running task: {:?}", task);

        let before_date = match task.details {
            Details::ClearCache { before_date } => before_date,
            _ => return Err("Invalid task details".into()),
        };

        let metadatas = self
            .metadata_storage
            .get_many_before_date(before_date, METADATA_LIMIT)?;

        if metadatas.is_empty() {
            task.status = TaskStatus::Completed;
            return Ok(task);
        }

        for mut metadata in metadatas {
            let derived_media_paths: Vec<_> = metadata
                .derived_medias
                .iter()
                .map(|derived_media| derived_media.path.clone())
                .collect();

            for path in derived_media_paths {
                self.cache_storage.delete(path.as_str()?)?;
                metadata.remove_derived_media(&path);
            }

            self.metadata_storage
                .save(metadata.path.as_str()?, metadata.clone())?;
        }

        Ok(task)
    }
}
