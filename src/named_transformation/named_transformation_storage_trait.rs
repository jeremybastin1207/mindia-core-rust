use std::error::Error;

use crate::named_transformation::{NamedTransformation, NamedTransformationMap};

pub trait NamedTransformationStorage: Send + Sync {
    fn get_all(&mut self) -> Result<NamedTransformationMap, Box<dyn Error>>;
    fn get_by_name(&mut self, name: &str) -> Result<Option<NamedTransformation>, Box<dyn Error>>;
    fn save(&mut self, apikey: NamedTransformation) -> Result<(), Box<dyn Error>>;
    fn delete(&mut self, apikey: &str) -> Result<(), Box<dyn Error>>;
}
