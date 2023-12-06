use std::path::{Path as StdPath, PathBuf};
use uuid::Uuid;

struct Path {
    path: PathBuf,
}

impl Path {
    fn new(path: String) -> Result<Self, &'static str> {
        let path = PathBuf::from(&path);
        if !path.is_absolute() {
            return Err("The path must be absolute");
        }
        Ok(Self { path })
    }

    fn generate(path: String) -> Result<Self, std::io::Error> {
        let mut path = PathBuf::from(path);
        let ext = path.extension().and_then(std::ffi::OsStr::to_str);
        let new_name = format!("{}.{}", Uuid::new_v4(), ext.unwrap_or(""));
        path.set_file_name(new_name);
        let path = path.canonicalize()?;
        Ok(Self { path })
    }

    fn folder(&self) -> Option<&str> {
        self.path.parent().and_then(|p| p.to_str())
    }

    fn basename(&self) -> &str {
        self.path.file_stem().and_then(|s| s.to_str())
    }

    fn extension(&self) -> &str {
        self.path.extension().and_then(|s| s.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_new() {
        let path = Path::new("/folder/test.txt".to_string());
        assert!(path.is_ok());
    }

    #[test]
    fn test_new_relative_path() {
        let path = Path::new("folder/test.txt".to_string());
        assert!(path.is_err());
    }

    #[test]
    fn test_generate() {
        let path = Path::generate("/test.txt".to_string());
        assert!(path.path.file_name().is_some());
    }

    #[test]
    fn test_folder() {
        let path = Path::new("/folder/test.txt".to_string()).unwrap();
        let current_dir = env::current_dir().unwrap();
        assert_eq!(path.folder(), Some(current_dir.to_str().unwrap()));
    }

    #[test]
    fn test_basename() {
        let path = Path::new("/folder/test.txt".to_string()).unwrap();
        assert_eq!(path.basename(), Some("test"));
    }

    #[test]
    fn test_extension() {
        let path = Path::new("/folder/test.txt".to_string()).unwrap();
        assert_eq!(path.extension(), Some("txt"));
    }
}
