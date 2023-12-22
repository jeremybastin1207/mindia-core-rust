use std::error::Error;

use super::{PipelineContext, PipelineStep};

type SourceFunc<T> = Box<dyn Fn(PipelineContext<T>) -> Result<PipelineContext<T>, Box<dyn Error>>>;

pub struct Source<T> {
    get: SourceFunc<T>,
}

impl<T> Source<T> {
    pub fn new(get: SourceFunc<T>) -> Self {
        Self { get }
    }
}

impl<T> PipelineStep<T> for Source<T> {
    fn execute(&self, context: PipelineContext<T>) -> Result<PipelineContext<T>, Box<dyn Error>> {
        (self.get)(context)
    }
}
