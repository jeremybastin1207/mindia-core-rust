use std::error::Error;
use std::sync::Arc;

use crate::transform::{ContextTransform, Transform};

pub struct TransformExecutor {}

impl TransformExecutor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transform(
        &self,
        mut context: ContextTransform,
        transforms: Vec<Box<dyn Transform>>,
    ) -> Result<ContextTransform, Box<dyn Error>> {
        for transform in &transforms {
            context = transform.transform(context)?;
        }

        Ok(context)
    }
}
