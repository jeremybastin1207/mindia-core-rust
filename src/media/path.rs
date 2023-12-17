use std::error::Error;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Path {
    path: PathBuf,
}

impl Path {
    pub fn new(path: String) -> Result<Self, &'static str> {
        let path = PathBuf::from(&path);
        Ok(Self { path })
    }

    pub fn generate(path: String) -> Result<Self, Box<dyn Error>> {
        let mut path = PathBuf::from(path);
        let ext = path.extension().and_then(std::ffi::OsStr::to_str);
        let new_name = format!("{}.{}", Uuid::new_v4(), ext.unwrap_or(""));
        path.set_file_name(new_name);
        let path = path.canonicalize()?;
        Ok(Self { path })
    }

    pub fn folder(&self) -> Option<&str> {
        self.path.parent().and_then(|p| p.to_str())
    }

    pub fn basename(&self) -> &str {
        self.path.file_stem().and_then(|s| s.to_str()).unwrap_or("")
    }

    pub fn extension(&self) -> &str {
        self.path.extension().and_then(|s| s.to_str()).unwrap_or("")
    }

    pub fn set_extension(&mut self, ext: &str) {
        self.path.set_extension(ext);
    }

    pub fn as_str(&self) -> Result<&str, &'static str> {
        self.path.to_str().ok_or("Invalid path")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let path = Path::new("/folder/test.txt".to_string());
        assert!(path.is_ok());
    }
}
