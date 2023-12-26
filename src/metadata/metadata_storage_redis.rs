use std::error::Error;

use crate::metadata::{Metadata, MetadataStorage};

const METADATA_PREFIX_KEY: &str = "metadata:";

pub struct RedisMetadataStorage {
    conn: redis::Connection,
}

impl RedisMetadataStorage {
    pub fn new(client: redis::Client) -> Result<Self, Box<dyn Error>> {
        let conn = client.get_connection()?;
        Ok(Self { conn })
    }
}

impl MetadataStorage for RedisMetadataStorage {
    fn get_by_path(&mut self, path: &str) -> Result<Option<Metadata>, Box<dyn Error>> {
        let key = format!("{}{}", METADATA_PREFIX_KEY, path);

        let result: Result<String, _> = redis::cmd("JSON.GET").arg(key).query(&mut self.conn);

        match result {
            Ok(data) => {
                let metadata: Metadata = serde_json::from_str(&data)?;
                Ok(Some(metadata))
            }
            Err(_) => Ok(None),
        }
    }

    fn save(&mut self, path: &str, metadata: Metadata) -> Result<(), Box<dyn Error>> {
        let metadata_str = serde_json::to_string(&metadata)?;

        let key = format!("{}{}", METADATA_PREFIX_KEY, path);

        redis::cmd("JSON.SET")
            .arg(key)
            .arg(".")
            .arg(metadata_str)
            .query(&mut self.conn)?;

        Ok(())
    }

    fn delete(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let key = format!("{}{}", METADATA_PREFIX_KEY, path);

        redis::cmd("DEL").arg(key).query(&mut self.conn)?;

        Ok(())
    }
}
