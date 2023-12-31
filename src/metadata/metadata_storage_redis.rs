use chrono::{DateTime, Utc};
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
        let key = format!("{}{}", METADATA_PREFIX_KEY, path);

        let result: Result<String, _> = redis::cmd("JSON.GET")
            .arg(key)
            .query(&mut self.conn.lock().unwrap());

        match result {
            Ok(data) => {
                let metadata: Metadata = serde_json::from_str(&data)?;
                Ok(Some(metadata))
            }
            Err(_) => Ok(None),
        }
    }

    fn get_many_before_date(
        &self,
        before_date: DateTime<Utc>,
        limit: u32,
    ) -> Result<Vec<Metadata>, Box<dyn Error>> {
        let mut cursor = 0;
        let mut metadata_before_instant = Vec::new();
        loop {
            let scan: (i64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(format!("{}*", METADATA_PREFIX_KEY))
                .arg("COUNT")
                .arg(limit)
                .query(&mut self.conn.lock().unwrap())?;

            cursor = scan.0;
            for key in scan.1 {
                let document: String = redis::cmd("JSON.GET")
                    .arg(&key)
                    .query(&mut self.conn.lock().unwrap())?;

                let metadata: Metadata = serde_json::from_str(&document)?;

                if metadata
                    .derived_medias
                    .iter()
                    .any(|dm| dm.created_at < before_date)
                {
                    metadata_before_instant.push(metadata);
                }
            }

            if cursor == 0 {
                break;
            }
        }

        Ok(metadata_before_instant)
    }

    fn save(&self, path: &str, metadata: Metadata) -> Result<(), Box<dyn Error>> {
        let metadata_str = serde_json::to_string(&metadata)?;

        let key = format!("{}{}", METADATA_PREFIX_KEY, path);

        redis::cmd("JSON.SET")
            .arg(key)
            .arg(".")
            .arg(metadata_str)
            .query(&mut self.conn.lock().unwrap())?;

        Ok(())
    }

    fn delete(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let key = format!("{}{}", METADATA_PREFIX_KEY, path);

        redis::cmd("DEL")
            .arg(key)
            .query(&mut self.conn.lock().unwrap())?;

        Ok(())
    }
}
