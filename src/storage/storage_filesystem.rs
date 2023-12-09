use std::fs;
use std::path::Path;

use crate::storage::storage_trait::Storage;

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

impl Storage for FilesystemStorage {
    fn upload(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        println!("Uploading to Filesystem");

        let path = Path::new(&self.mount_dir).join("file.jpeg");

        fs::write(path, data)?;

        Ok(())
    }
}
