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
        let exists: bool = redis::cmd("EXISTS")
            .arg(API_KEYS_KEY)
            .query(&mut self.conn)?;

        if !exists {
            redis::cmd("JSON.SET")
                .arg(API_KEYS_KEY)
                .arg(".")
                .arg("{}")
                .query(&mut self.conn)?;
        }

        Ok(())
    }
}

impl ApiKeyStorage for RedisApiKeyStorage {
    fn get_all(&mut self) -> Result<ApiKeyMap, Box<dyn Error>> {
        let result: String = redis::cmd("JSON.GET")
            .arg(API_KEYS_KEY)
            .query(&mut self.conn)?;

        serde_json::from_str(&result).map_err(|e| e.into())
    }

    fn get_by_name(&mut self, name: &str) -> Result<Option<ApiKey>, Box<dyn Error>> {
        let result: Option<String> = redis::cmd("JSON.GET")
            .arg(API_KEYS_KEY)
            .arg(name)
            .query(&mut self.conn)?;

        result
            .map(|json| serde_json::from_str(&json))
            .transpose()
            .map_err(|e| e.into())
    }

    fn get_by_key(&mut self, key: &str) -> Result<Option<ApiKey>, Box<dyn Error>> {
        let apikeys = self.get_all()?;

        let result = apikeys
            .into_iter()
            .find(|(_, apikey)| apikey.key == key)
            .map(|(_, apikey)| apikey);

        Ok(result)
    }

    fn save(&mut self, apikey: ApiKey) -> Result<(), Box<dyn Error>> {
        let apikey_json = serde_json::to_string(&apikey)?;

        redis::cmd("JSON.SET")
            .arg(API_KEYS_KEY)
            .arg(apikey.name)
            .arg(apikey_json)
            .query(&mut self.conn)?;

        Ok(())
    }

    fn delete(&mut self, apikey_name: &str) -> Result<(), Box<dyn Error>> {
        redis::cmd("JSON.DEL")
            .arg(API_KEYS_KEY)
            .arg(apikey_name)
            .query(&mut self.conn)?;

        Ok(())
    }
}
