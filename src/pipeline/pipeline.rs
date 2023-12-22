use super::{PipelineStep, Sinker, Source};

pub struct Pipeline<T> {
    source: Source<T>,
    sinker: Sinker<T>,
    steps: Vec<Box<dyn PipelineStep<T>>>,
}

impl<T> Pipeline<T> {
    pub fn new(
        source: Source<T>,
        sinker: Sinker<T>,
        steps: Vec<Box<dyn PipelineStep<T>>>,
    ) -> Pipeline<T> {
        Pipeline {
            source,
            sinker,
            steps,
        }
    }

    pub fn steps(&self) -> Vec<&dyn PipelineStep<T>> {
        let mut steps: Vec<&dyn PipelineStep<T>> = vec![&self.source as &dyn PipelineStep<T>];

        for step in &self.steps {
            steps.push(step.as_ref());
        }

        steps.push(&self.sinker as &dyn PipelineStep<T>);

        steps
    }
}
