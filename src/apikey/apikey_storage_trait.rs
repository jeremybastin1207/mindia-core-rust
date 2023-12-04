use std::error::Error;

use crate::apikey::apikey::{ApiKey, ApiKeyMap};

pub trait ApiKeyStorage: Send + Sync {
    fn get_all(&mut self) -> Result<ApiKeyMap, Box<dyn Error>>;
    fn get_by_name(&mut self, name: &str) -> Result<Option<ApiKey>, Box<dyn Error>>;
    fn get_by_key(&mut self, key: &str) -> Result<Option<ApiKey>, Box<dyn Error>>;
    fn save(&mut self, apikey: ApiKey) -> Result<(), Box<dyn Error>>;
    fn delete(&mut self, apikey: &str) -> Result<(), Box<dyn Error>>;
}
