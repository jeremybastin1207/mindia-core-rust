use redis::Commands;
use serde_json::{from_str, json, to_string};
use std::collections::HashMap;
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
        let res: Option<String> = self.conn.get(NAMED_TRANSFORMATIONS_KEY)?;
        match res {
            Some(_) => Ok(()),
            None => {
                let empty_json = json!({});
                self.conn
                    .set(NAMED_TRANSFORMATIONS_KEY, empty_json.to_string())?;
                Ok(())
            }
        }
    }
}

impl NamedTransformationStorage for RedisNamedTransformationStorage {
    fn get_all(&mut self) -> Result<NamedTransformationMap, Box<dyn Error>> {
        let res: Option<String> = self.conn.get(NAMED_TRANSFORMATIONS_KEY)?;
        match res {
            Some(val) => {
                let named_transformations: NamedTransformationMap = serde_json::from_str(&val)?;
                Ok(named_transformations)
            }
            None => Err("No named transformation found".into()),
        }
    }

    fn get_by_name(&mut self, name: &str) -> Result<Option<NamedTransformation>, Box<dyn Error>> {
        let named_transformations = self.get_all()?;
        match named_transformations.get(name) {
            Some(named_transformation) => Ok(Some(named_transformation.clone())),
            None => Err("Named transformation not found".into()),
        }
    }

    fn save(&mut self, named_transformation: NamedTransformation) -> Result<(), Box<dyn Error>> {
        let named_transformations_json: String = self.conn.get(NAMED_TRANSFORMATIONS_KEY)?;
        let mut named_transformations: HashMap<String, NamedTransformation> =
            from_str(&named_transformations_json)?;

        named_transformations.insert(named_transformation.name.clone(), named_transformation);

        let updated_named_transformations_json = to_string(&named_transformations)?;
        self.conn.set(
            NAMED_TRANSFORMATIONS_KEY,
            updated_named_transformations_json,
        )?;

        Ok(())
    }

    fn delete(&mut self, named_transformation_name: &str) -> Result<(), Box<dyn Error>> {
        let named_transformations_json: String = self.conn.get(NAMED_TRANSFORMATIONS_KEY)?;
        let mut named_transformations: HashMap<String, NamedTransformation> =
            from_str(&named_transformations_json)?;

        named_transformations.remove(named_transformation_name);

        let updated_named_transformations_json = to_string(&named_transformations)?;
        self.conn.set(
            NAMED_TRANSFORMATIONS_KEY,
            updated_named_transformations_json,
        )?;

        Ok(())
    }
}
