use redis::Connection;
use std::error::Error;
use std::sync::{Arc, Mutex};

const NAMED_TRANSFORMATIONS_KEY: &str = "internal:configuration:named_transformations";

use super::{NamedTransformation, NamedTransformationMap, NamedTransformationStorage};

pub struct RedisNamedTransformationStorage {
    conn: Arc<Mutex<Connection>>,
}

impl RedisNamedTransformationStorage {
    pub fn new(conn: Connection) -> Result<Self, Box<dyn Error>> {
        let mut storage = RedisNamedTransformationStorage {
            conn: Arc::new(Mutex::new(conn)),
        };
        storage.init()?;
        Ok(storage)
    }

    fn init(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let exists: bool = redis::cmd("EXISTS")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .query(&mut conn)?;

        if !exists {
            redis::cmd("JSON.SET")
                .arg(NAMED_TRANSFORMATIONS_KEY)
                .arg(".")
                .arg("{}")
                .query(&mut conn)?;
        }

        Ok(())
    }
}

impl NamedTransformationStorage for RedisNamedTransformationStorage {
    fn get_all(&self) -> Result<NamedTransformationMap, Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let result: String = redis::cmd("JSON.GET")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .query(&mut conn)?;

        serde_json::from_str(&result).map_err(|e| e.into())
    }

    fn get_by_name(&self, name: &str) -> Result<Option<NamedTransformation>, Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let result: Option<String> = redis::cmd("JSON.GET")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .arg(name)
            .query(&mut conn)?;

        result
            .map(|json| serde_json::from_str(&json))
            .transpose()
            .map_err(|e| e.into())
    }

    fn save(&self, named_transformation: NamedTransformation) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        let transformation_json = serde_json::to_string(&named_transformation)?;

        redis::cmd("JSON.SET")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .arg(named_transformation.name)
            .arg(transformation_json)
            .query(&mut conn)?;

        Ok(())
    }

    fn delete(&self, named_transformation_name: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.lock().unwrap();

        redis::cmd("JSON.DEL")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .arg(named_transformation_name)
            .query(&mut conn)?;

        Ok(())
    }
}
