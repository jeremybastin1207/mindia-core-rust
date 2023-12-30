use std::error::Error;

use crate::apikey::apikey::{ApiKey, ApiKeyMap};

pub trait ApiKeyStorage: Send + Sync {
    fn get_all(&self) -> Result<ApiKeyMap, Box<dyn Error>>;
    fn get_by_name(&self, name: &str) -> Result<Option<ApiKey>, Box<dyn Error>>;
    fn get_by_key(&self, key: &str) -> Result<Option<ApiKey>, Box<dyn Error>>;
    fn save(&self, apikey: ApiKey) -> Result<(), Box<dyn Error>>;
    fn delete(&self, apikey: &str) -> Result<(), Box<dyn Error>>;
}
