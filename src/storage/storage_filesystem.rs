use bytes::Bytes;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::storage::storage_trait::FileStorage;

pub struct FilesystemStorage {
    pub mount_dir: String,
}

impl FilesystemStorage {
    pub fn new(mount_dir: &str) -> Self {
        Self {
            mount_dir: mount_dir.to_string(),
        }
    }
}

impl FileStorage for FilesystemStorage {
    fn upload(&self, mut path: &str, data: Bytes) -> Result<(), Box<dyn Error>> {
        path = path.strip_prefix("/").unwrap_or(path);

        let full_path = Path::new(&self.mount_dir).join(path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&full_path, data)?;

        Ok(())
    }
}
