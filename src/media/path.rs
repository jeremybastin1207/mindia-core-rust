use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::error::Error;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Path {
    raw_path: String,
}

impl Path {
    pub fn new(raw_path: &str) -> Result<Self, &'static str> {
        if raw_path == "" || !raw_path.starts_with('/') {
            return Err("Invalid path");
        }

        let path = Self{  raw_path: raw_path.to_string() };

        Uuid::parse_str(path.basename()).map_err(|_| "Invalid path")?;

        Ok(path)
    }

    pub fn add_suffix_to_filename(&self, suffix: &str) -> Result<Self, Box<dyn Error>> {
        let mut path = PathBuf::from(self.raw_path.as_str());
        let ext = path.extension().and_then(std::ffi::OsStr::to_str);
        let raw_path = format!("{}/{}-{}.{}", self.folder(), self.basename(), suffix, ext.unwrap_or(""));
        Ok(Self { raw_path })
    }

    pub fn folder(&self) -> String {
        let components: Vec<&str> = self.raw_path.split('/').collect();
        if components.len() > 1 {
            return components[..components.len() - 1].join("/");
        }
        "".to_string()
    }

    pub fn basename(&self) -> &str {
        let components: Vec<&str> = self.raw_path.split('/').collect();
        if let Some(last) = components.last() {
            let file_parts: Vec<&str> = last.split('.').collect();
            if file_parts.len() > 1 {
                return file_parts.first().unwrap_or(&"");
            }
        }
        ""
    }

    pub fn extension(&self) -> &str {
        let components: Vec<&str> = self.raw_path.split('/').collect();
        if let Some(last) = components.last() {
            let file_parts: Vec<&str> = last.split('.').collect();
            if file_parts.len() > 1 {
                return file_parts.last().unwrap_or(&"");
            }
        }
        ""
    }

    pub fn set_extension(&mut self, ext: &str) {
        let mut joined_path = String::new();

        let mut components: Vec<&str> = self.raw_path.split('/').collect();
        if let Some(last) = components.last_mut() {
            let mut file_parts: Vec<&str> = last.split('.').collect();
            if file_parts.len() > 1 {
                file_parts.pop();
            }
            file_parts.push(ext);
            joined_path = file_parts.join(".");
            *last = &joined_path;
        }
        self.raw_path = components.join("/");
    }

    pub fn as_str(&self) -> &str {
        self.raw_path.as_str()
    }
}

pub fn generate_path(path: &str) -> Result<Path, &str> {
    let mut path = PathBuf::from(path);
    let ext = path.extension().and_then(std::ffi::OsStr::to_str);
    let new_name = format!("{}.{}", Uuid::new_v4(), ext.unwrap_or(""));
    path.set_file_name(new_name);
    let path = "/".to_string() + path.to_str().unwrap().replace("\\", "/").as_str();
    Path::new(path.as_str())
}

impl From<String> for Path {
    fn from(s: String) -> Self {
        Self::new(&s).unwrap_or_else(|_| Self {
            raw_path: String::new(),
        })
    }
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = self.raw_path.as_str();
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        // Deserialize the string
        let s = String::deserialize(deserializer)?;

        Ok(Self { raw_path: s })
    }
}

#[cfg(test)]
mod tests {
    use super::{generate_path, Path};
    use super::Uuid;

    #[test]
    fn test_new() {
        let path = Path::new("/folder/bbd2fa99-f35e-4062-92eb-9d26caa943ae.txt");
        assert!(path.is_ok());

        let invalid_path = Path::new("");
        assert!(invalid_path.is_err());
    }

    #[test]
    fn test_add_suffix_to_filename() {
        let path = Path::new("/folder/bbd2fa99-f35e-4062-92eb-9d26caa943ae.txt").unwrap();
        let derived_path = path.add_suffix_to_filename("suffix").unwrap();
        assert_eq!(derived_path.as_str(), "/folder/bbd2fa99-f35e-4062-92eb-9d26caa943ae-suffix.txt");
    }

    #[test]
    fn test_generate() {
        let path = generate_path("/folder/bbd2fa99-f35e-4062-92eb-9d26caa943ae.txt").unwrap();
        let uuid = Uuid::parse_str(path.basename());
        assert!(uuid.is_ok());
    }

    #[test]
    fn test_folder() {
        let path = Path::new("/folder/bbd2fa99-f35e-4062-92eb-9d26caa943ae.txt").unwrap();
        assert_eq!(path.folder(), "/folder".to_string());
    }

    #[test]
    fn test_basename() {
        let path = Path::new("/folder/bbd2fa99-f35e-4062-92eb-9d26caa943ae.txt").unwrap();
        assert_eq!(path.basename(), "test");
    }

    #[test]
    fn test_extension() {
        let path = Path::new("/folder/bbd2fa99-f35e-4062-92eb-9d26caa943ae.txt").unwrap();
        assert_eq!(path.extension(), "txt");
    }

    #[test]
    fn test_set_extension() {
        let mut path = Path::new("/folder/bbd2fa99-f35e-4062-92eb-9d26caa943ae.txt").unwrap();
        path.set_extension("jpg");
        assert_eq!(path.extension(), "jpg");
        assert_eq!(path.as_str(), "/folder/bbd2fa99-f35e-4062-92eb-9d26caa943ae.jpg");
    }

    #[test]
    fn test_as_str() {
        let path = Path::new("/folder/test.txt").unwrap();
        assert_eq!(path.as_str(), "/folder/test.txt");
    }
}
