use std::error::Error;

const NAMED_TRANSFORMATIONS_KEY: &str = "internal:configuration:named_transformations";

use crate::named_transformation::{
    NamedTransformation, NamedTransformationMap, NamedTransformationStorage,
};

pub struct RedisNamedTransformationStorage {
    conn: redis::Connection,
}

impl RedisNamedTransformationStorage {
    pub fn new(client: redis::Client) -> Result<Self, Box<dyn Error>> {
        let conn = client.get_connection()?;
        let mut storage = RedisNamedTransformationStorage { conn };
        storage.init()?;
        Ok(storage)
    }

    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let exists: bool = redis::cmd("EXISTS")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .query(&mut self.conn)?;

        if !exists {
            redis::cmd("JSON.SET")
                .arg(NAMED_TRANSFORMATIONS_KEY)
                .arg(".")
                .arg("{}")
                .query(&mut self.conn)?;
        }

        Ok(())
    }
}

impl NamedTransformationStorage for RedisNamedTransformationStorage {
    fn get_all(&mut self) -> Result<NamedTransformationMap, Box<dyn Error>> {
        let result: String = redis::cmd("JSON.GET")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .query(&mut self.conn)?;

        serde_json::from_str(&result).map_err(|e| e.into())
    }

    fn get_by_name(&mut self, name: &str) -> Result<Option<NamedTransformation>, Box<dyn Error>> {
        let result: Option<String> = redis::cmd("JSON.GET")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .arg(name)
            .query(&mut self.conn)?;

        result
            .map(|json| serde_json::from_str(&json))
            .transpose()
            .map_err(|e| e.into())
    }

    fn save(&mut self, named_transformation: NamedTransformation) -> Result<(), Box<dyn Error>> {
        let transformation_json = serde_json::to_string(&named_transformation)?;

        redis::cmd("JSON.SET")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .arg(named_transformation.name)
            .arg(transformation_json)
            .query(&mut self.conn)?;

        Ok(())
    }

    fn delete(&mut self, named_transformation_name: &str) -> Result<(), Box<dyn Error>> {
        redis::cmd("JSON.DEL")
            .arg(NAMED_TRANSFORMATIONS_KEY)
            .arg(named_transformation_name)
            .query(&mut self.conn)?;

        Ok(())
    }
}
