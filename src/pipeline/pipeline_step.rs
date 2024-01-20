use std::error::Error;
use async_trait::async_trait;

use super::PipelineContext;

#[async_trait]
pub trait PipelineStep<T: std::marker::Send> {
    async fn execute(&self, context: PipelineContext<T>) -> Result<PipelineContext<T>, Box<dyn Error>>;
}
