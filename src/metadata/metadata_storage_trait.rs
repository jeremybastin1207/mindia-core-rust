use std::error::Error;

use crate::metadata::Metadata;

pub trait MetadataStorage: Send + Sync {
    fn get_by_path(&mut self, path: &str) -> Result<Option<Metadata>, Box<dyn Error>>;
    fn save(&mut self, path: &str, metadata: Metadata) -> Result<(), Box<dyn Error>>;
    fn delete(&mut self, path: &str) -> Result<(), Box<dyn Error>>;
}
