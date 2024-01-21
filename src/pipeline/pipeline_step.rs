use std::error::Error;
use async_trait::async_trait;


#[async_trait]
pub trait PipelineStep<T: Send> {
    async fn execute(&self, context: T) -> Result<T, Box<dyn Error>>;
}
