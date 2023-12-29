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
    fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn Error>> {
        let path = path.strip_prefix("/").unwrap_or(path);
        let full_path = Path::new(&self.mount_dir).join(path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&full_path, data)?;

        Ok(())
    }

    fn download(&self, path: &str) -> Result<Option<Bytes>, Box<dyn Error>> {
        let path = path.strip_prefix("/").unwrap_or(path);
        let full_path = Path::new(&self.mount_dir).join(path);

        match fs::read(&full_path) {
            Ok(data) => Ok(Some(Bytes::from(data))),
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn delete(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let path = path.strip_prefix("/").unwrap_or(path);
        let full_path = Path::new(&self.mount_dir).join(path);

        fs::remove_file(&full_path)?;

        Ok(())
    }
}
