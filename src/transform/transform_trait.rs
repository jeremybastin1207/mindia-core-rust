use std::error::Error;

use crate::transform::ContextTransform;

pub trait Transformer {
    fn transform(&self, context: ContextTransform) -> Result<ContextTransform, Box<dyn Error>>;
}
