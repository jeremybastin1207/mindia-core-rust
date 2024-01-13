use bytes::Bytes;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::storage::storage_trait::FileStorage;

pub struct FilesystemStorage {
    pub mount_dir: String,
}

impl FilesystemStorage {
    pub fn new(mount_dir: String) -> Self {
        Self {
            mount_dir,
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

    fn move_(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
        let src = src.strip_prefix("/").unwrap_or(src);
        let dst = dst.strip_prefix("/").unwrap_or(dst);
        let src_full_path = Path::new(&self.mount_dir).join(src);
        let dst_full_path = Path::new(&self.mount_dir).join(dst);

        fs::rename(&src_full_path, &dst_full_path)?;

        Ok(())
    }

    fn copy(&self, src: &str, dst: &str) -> Result<(), Box<dyn Error>> {
        let src = src.strip_prefix("/").unwrap_or(src);
        let dst = dst.strip_prefix("/").unwrap_or(dst);
        let src_full_path = Path::new(&self.mount_dir).join(src);
        let dst_full_path = Path::new(&self.mount_dir).join(dst);

        fs::copy(&src_full_path, &dst_full_path)?;

        Ok(())
    }

    fn delete(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let path = path.strip_prefix("/").unwrap_or(path);
        let full_path = Path::new(&self.mount_dir).join(path);

        fs::remove_file(&full_path)?;

        Ok(())
    }
}
