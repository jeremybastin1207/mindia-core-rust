use std::error::Error;

use crate::pipeline::{PipelineContext, PipelineStep};

type SinkerFunc<T> = Box<dyn Fn(&PipelineContext<T>) -> Result<(), Box<dyn Error>>>;

pub struct Sinker<T> {
    sink: SinkerFunc<T>,
}

impl<T> Sinker<T> {
    pub fn new(sink: SinkerFunc<T>) -> Self {
        Self { sink }
    }
}

impl<T> PipelineStep<T> for Sinker<T> {
    fn execute(&self, context: PipelineContext<T>) -> Result<PipelineContext<T>, Box<dyn Error>> {
        (self.sink)(&context)?;

        Ok(context)
    }
}
