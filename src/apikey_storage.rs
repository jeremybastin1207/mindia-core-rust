extern crate redis;
extern crate serde_json;

use redis::Commands;
use serde_json::{json, Value};
use std::error::Error;

const API_KEYS_KEY: &str = "internal:configuration:api_keys";

pub trait ApiKeyStorage {
    fn get_by_name(&self, name: &str) -> Result<ApiKey, Box<dyn Error>>;
    fn get_by_key(&self, key: &str) -> Result<ApiKey, Box<dyn Error>>;
    fn get_all(&self) -> Result<ApiKeyMap, Box<dyn Error>>;
    fn save(&self, apikey: ApiKey) -> Result<(), Box<dyn Error>>;
    fn delete(&self, apikey: &str) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiKey {
    name: String,
    key: String,
}

type ApiKeyMap = HashMap<String, ApiKey>;

pub struct RedisApiKeyStorage {
    conn: redis::Connection,
}

impl RedisApiKeyStorage {
    pub fn new(client: redis::Client) -> Result<Self, Box<dyn Error>> {
        let conn = client.get_connection()?;
        let storage = ApiKeyStorage { conn };
        storage.init()?;
        Ok(storage)
    }

    fn init(&self) -> Result<(), Box<dyn Error>> {
        let res: Option<String> = self.conn.get(API_KEYS_KEY)?;
        match res {
            Some(_) => Ok(()),
            None => {
                let empty_json = json!({});
                self.conn.set(API_KEYS_KEY, empty_json.to_string())?;
                Ok(())
            }
        }
    }
}

impl ApiKeyStorage for RedisApiKeyStorage {
    fn get_all(&self) -> Result<ApiKeyMap, Box<dyn Error>> {
        let res: Option<String> = self.conn.get(API_KEYS_KEY)?;
        match res {
            Some(val) => {
                let api_keys: ApiKeyMap = serde_json::from_str(&val)?;
                Ok(api_keys)
            }
            None => Err("No API keys found".into()),
        }
    }

    fn get_by_name(&self, name: &str) -> Result<Option<ApiKey>, Box<dyn Error>> {
        let api_keys = self.get_all()?;
        match api_keys.get(name) {
            Some(api_key) => Ok(Some(api_key.clone())),
            None => Err("API key not found".into()),
        }
    }

    fn get_by_key(&self, key: &str) -> Result<Option<ApiKey>, Box<dyn Error>> {
        let api_keys = self.get_all()?;
        for api_key in api_keys.values() {
            if api_key.key == key {
                return Ok(Some(api_key.clone()));
            }
        }
        Err("API key not found".into())
    }
}
