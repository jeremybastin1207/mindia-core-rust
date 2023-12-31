use chrono::{DateTime, Utc};
use std::error::Error;

use crate::metadata::Metadata;

pub trait MetadataStorage: Send + Sync {
    fn get_by_path(&self, path: &str) -> Result<Option<Metadata>, Box<dyn Error>>;
    fn get_many_before_date(
        &self,
        before_date: DateTime<Utc>,
        limit: u32,
    ) -> Result<Vec<Metadata>, Box<dyn Error>>;
    fn save(&self, path: &str, metadata: Metadata) -> Result<(), Box<dyn Error>>;
    fn delete(&self, path: &str) -> Result<(), Box<dyn Error>>;
}
