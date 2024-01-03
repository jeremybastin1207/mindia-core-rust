use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::error::Error;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Path {
    path: PathBuf,
}

impl Path {
    pub fn new(path: &str) -> Result<Self, &'static str> {
        let path = PathBuf::from(path);
        Ok(Self { path })
    }

    pub fn derive_with_suffix(&self, suffix: &str) -> Result<Self, Box<dyn Error>> {
        let mut path = self.path.clone();
        let ext = path.extension().and_then(std::ffi::OsStr::to_str);
        let new_name = format!("{}-{}.{}", self.basename(), suffix, ext.unwrap_or(""));
        path.set_file_name(new_name);
        Ok(Self { path })
    }

    pub fn generate(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut path = PathBuf::from(path);
        let ext = path.extension().and_then(std::ffi::OsStr::to_str);
        let new_name = format!("{}.{}", Uuid::new_v4(), ext.unwrap_or(""));
        path.set_file_name(new_name);
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

impl From<String> for Path {
    fn from(s: String) -> Self {
        Self::new(&s).unwrap_or_else(|_| Self {
            path: PathBuf::new(),
        })
    }
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.path.to_str().ok_or(serde::ser::Error::custom(
            "Could not convert path to string",
        ))?;
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

        // Convert the string to a PathBuf
        let path = PathBuf::from(&s);

        Ok(Self { path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let path = Path::new("/folder/test.txt");
        assert!(path.is_ok());
    }
}
