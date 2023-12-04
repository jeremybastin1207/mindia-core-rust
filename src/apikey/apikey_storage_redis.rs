use redis::Commands;
use serde_json::{from_str, json, to_string};
use std::collections::HashMap;
use std::error::Error;

const API_KEYS_KEY: &str = "internal:configuration:api_keys";

use crate::apikey::{ApiKey, ApiKeyMap, ApiKeyStorage};

pub struct RedisApiKeyStorage {
    conn: redis::Connection,
}

impl RedisApiKeyStorage {
    pub fn new(client: redis::Client) -> Result<Self, Box<dyn Error>> {
        let conn = client.get_connection()?;
        let mut storage = RedisApiKeyStorage { conn };
        storage.init()?;
        Ok(storage)
    }

    fn init(&mut self) -> Result<(), Box<dyn Error>> {
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
    fn get_all(&mut self) -> Result<ApiKeyMap, Box<dyn Error>> {
        let res: Option<String> = self.conn.get(API_KEYS_KEY)?;
        match res {
            Some(val) => {
                let api_keys: ApiKeyMap = serde_json::from_str(&val)?;
                Ok(api_keys)
            }
            None => Err("No API keys found".into()),
        }
    }

    fn get_by_name(&mut self, name: &str) -> Result<Option<ApiKey>, Box<dyn Error>> {
        let api_keys = self.get_all()?;
        match api_keys.get(name) {
            Some(api_key) => Ok(Some(api_key.clone())),
            None => Err("API key not found".into()),
        }
    }

    fn get_by_key(&mut self, key: &str) -> Result<Option<ApiKey>, Box<dyn Error>> {
        let api_keys = self.get_all()?;
        for api_key in api_keys.values() {
            if api_key.key == key {
                return Ok(Some(api_key.clone()));
            }
        }
        Err("API key not found".into())
    }

    fn save(&mut self, apikey: ApiKey) -> Result<(), Box<dyn Error>> {
        let apikeys_json: String = self.conn.get(API_KEYS_KEY)?;
        let mut apikeys: HashMap<String, ApiKey> = from_str(&apikeys_json)?;

        apikeys.insert(apikey.name.clone(), apikey);

        let updated_apikeys_json = to_string(&apikeys)?;
        self.conn.set(API_KEYS_KEY, updated_apikeys_json)?;

        Ok(())
    }

    fn delete(&mut self, apikey_name: &str) -> Result<(), Box<dyn Error>> {
        let apikeys_json: String = self.conn.get(API_KEYS_KEY)?;
        let mut apikeys: HashMap<String, ApiKey> = from_str(&apikeys_json)?;

        apikeys.remove(apikey_name);

        let updated_apikeys_json = to_string(&apikeys)?;
        self.conn.set(API_KEYS_KEY, updated_apikeys_json)?;

        Ok(())
    }
}
