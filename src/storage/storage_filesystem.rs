use bytes::Bytes;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::storage::storage_trait::FileStorage;

pub struct FilesystemStorage {
    pub mount_dir: String,
}

impl FilesystemStorage {
    pub fn new() -> Self {
        Self {
            mount_dir: String::from("./mnt"),
        }
    }
}

impl FileStorage for FilesystemStorage {
    fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn Error>> {
        println!("Uploading to Filesystem");

        let dir_path = Path::new(&self.mount_dir);

        let path = path.strip_prefix("/").unwrap_or(path);

        let full_path = Path::new(&self.mount_dir).join(path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&full_path, data)?;

        Ok(())
    }
}
