use redis::Connection;
use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::metadata::{Metadata, MetadataStorage};

const METADATA_PREFIX_KEY: &str = "metadata:";

pub struct RedisMetadataStorage {
    conn: Arc<Mutex<Connection>>,
}

impl RedisMetadataStorage {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl MetadataStorage for RedisMetadataStorage {
    fn get_by_path(&self, path: &str) -> Result<Option<Metadata>, Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let key = format!("{}{}", METADATA_PREFIX_KEY, path);

        let result: Result<String, _> = redis::cmd("JSON.GET").arg(key).query(&mut conn);

        match result {
            Ok(data) => {
                let metadata: Metadata = serde_json::from_str(&data)?;
                Ok(Some(metadata))
            }
            Err(_) => Ok(None),
        }
    }

    fn get_all(&self) -> Result<Vec<Metadata>, Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();
        let mut cursor = 0;
        let mut metadatas = Vec::new();

        loop {
            let scan: (i64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(format!("{}*", METADATA_PREFIX_KEY))
                .query(&mut conn)?;

            cursor = scan.0;

            for key in scan.1 {
                let metadata_str: String = redis::cmd("JSON.GET").arg(key).query(&mut conn)?;

                let metadata: Metadata = serde_json::from_str(&metadata_str)?;

                metadatas.push(metadata);
            }

            if cursor == 0 {
                break;
            }
        }

        Ok(metadatas)
    }

    fn save(&self, path: &str, metadata: Metadata) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let metadata_str = serde_json::to_string(&metadata)?;

        let key = format!("{}{}", METADATA_PREFIX_KEY, path);

        redis::cmd("JSON.SET")
            .arg(key)
            .arg(".")
            .arg(metadata_str)
            .query(&mut conn)?;

        Ok(())
    }

    fn delete(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let key = format!("{}{}", METADATA_PREFIX_KEY, path);

        redis::cmd("DEL").arg(key).query(&mut conn)?;

        Ok(())
    }
}
