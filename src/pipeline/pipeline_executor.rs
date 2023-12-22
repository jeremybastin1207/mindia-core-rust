use std::error::Error;

use super::{Pipeline, PipelineContext};

pub struct PipelineExecutor {}

impl PipelineExecutor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute<T: Default>(
        &self,
        pipeline: Pipeline<T>,
    ) -> Result<PipelineContext<T>, Box<dyn Error>> {
        let mut context = PipelineContext::<T>::new(T::default());

        for step in pipeline.steps() {
            context = step.execute(context)?;
        }

        Ok(context)
    }
}
