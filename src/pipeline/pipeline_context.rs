#[derive(Debug, Clone)]
pub struct PipelineContext<T> {
    pub attributes: T,
}

impl<T> PipelineContext<T> {
    pub fn new(attributes: T) -> Self {
        Self { attributes }
    }
}
