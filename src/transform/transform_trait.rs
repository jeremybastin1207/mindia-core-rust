use std::error::Error;

use crate::transform::ContextTransform;

pub trait Transform {
    fn transform(&self, context: ContextTransform) -> Result<ContextTransform, Box<dyn Error>>;
}
