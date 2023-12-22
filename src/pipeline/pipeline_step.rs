use std::error::Error;

use super::PipelineContext;

pub trait PipelineStep<T> {
    fn execute(&self, context: PipelineContext<T>) -> Result<PipelineContext<T>, Box<dyn Error>>;
}
