#[cfg(test)]
use mockall::automock;

use std::error::Error;

use super::{NamedTransformation, NamedTransformationMap};

#[cfg_attr(test, automock)]
pub trait NamedTransformationStorage: Send + Sync {
    fn get_all(&mut self) -> Result<NamedTransformationMap, Box<dyn Error>>;
    fn get_by_name(&mut self, name: &str) -> Result<Option<NamedTransformation>, Box<dyn Error>>;
    fn save(&mut self, named_transformation: NamedTransformation) -> Result<(), Box<dyn Error>>;
    fn delete(&mut self, named_transformation: &str) -> Result<(), Box<dyn Error>>;
}
