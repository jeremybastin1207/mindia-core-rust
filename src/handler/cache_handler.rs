use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::metadata::MetadataStorage;
use crate::scheduler::{Details, Task, TaskExecutor, TaskStatus};
// use crate::scheduler::{Details, Task, TaskExecutor, TaskStatus};
use crate::storage::FileStorage;

const METADATA_LIMIT: u32 = 100;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClearCacheTaskDetails {
    pub before_created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct CacheHandler {
    cache_storage: Arc<Mutex<dyn FileStorage>>,
    metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
}

impl CacheHandler {
    pub fn new(
        cache_storage: Arc<Mutex<dyn FileStorage>>,
        metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
    ) -> Self {
        Self {
            cache_storage,
            metadata_storage,
        }
    }
}

#[async_trait]
impl TaskExecutor for CacheHandler {
    async fn run(&self, mut task: Task) -> Result<Task, Box<dyn Error>> {
        println!("Running task: {:?}", task);

        let before_date = match task.details {
            Details::ClearCache { before_date } => before_date,
        };

        let metadatas = self
            .metadata_storage
            .lock()
            .await
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
                self.cache_storage
                    .lock()
                    .await
                    .delete(path.as_str())
                    .await?;
                metadata.remove_derived_media(&path);
            }

            self.metadata_storage
                .lock()
                .await
                .save(metadata.path.as_str(), metadata.clone())?;
        }

        Ok(task)
    }
}
