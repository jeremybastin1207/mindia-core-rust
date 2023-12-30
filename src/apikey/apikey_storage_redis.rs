use redis::Connection;
use std::error::Error;
use std::sync::{Arc, Mutex};

const API_KEYS_KEY: &str = "internal:configuration:api_keys";

use crate::apikey::{ApiKey, ApiKeyMap, ApiKeyStorage};

pub struct RedisApiKeyStorage {
    conn: Arc<Mutex<Connection>>,
}

impl RedisApiKeyStorage {
    pub fn new(conn: Connection) -> Result<Self, Box<dyn Error>> {
        let mut storage = RedisApiKeyStorage {
            conn: Arc::new(Mutex::new(conn)),
        };
        storage.init()?;
        Ok(storage)
    }

    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let exists: bool = redis::cmd("EXISTS").arg(API_KEYS_KEY).query(&mut conn)?;

        if !exists {
            redis::cmd("JSON.SET")
                .arg(API_KEYS_KEY)
                .arg(".")
                .arg("{}")
                .query(&mut conn)?;
        }

        Ok(())
    }
}

impl ApiKeyStorage for RedisApiKeyStorage {
    fn get_all(&self) -> Result<ApiKeyMap, Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let result: String = redis::cmd("JSON.GET").arg(API_KEYS_KEY).query(&mut conn)?;

        serde_json::from_str(&result).map_err(|e| e.into())
    }

    fn get_by_name(&self, name: &str) -> Result<Option<ApiKey>, Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let result: Option<String> = redis::cmd("JSON.GET")
            .arg(API_KEYS_KEY)
            .arg(name)
            .query(&mut *conn)?;

        result
            .map(|json| serde_json::from_str(&json))
            .transpose()
            .map_err(|e| e.into())
    }

    fn get_by_key(&self, key: &str) -> Result<Option<ApiKey>, Box<dyn Error>> {
        let apikeys = self.get_all()?;

        let result = apikeys
            .into_iter()
            .find(|(_, apikey)| apikey.key == key)
            .map(|(_, apikey)| apikey);

        Ok(result)
    }

    fn save(&self, apikey: ApiKey) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let apikey_json = serde_json::to_string(&apikey)?;

        redis::cmd("JSON.SET")
            .arg(API_KEYS_KEY)
            .arg(apikey.name)
            .arg(apikey_json)
            .query(&mut conn)?;

        Ok(())
    }

    fn delete(&self, apikey_name: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        redis::cmd("JSON.DEL")
            .arg(API_KEYS_KEY)
            .arg(apikey_name)
            .query(&mut conn)?;

        Ok(())
    }
}
