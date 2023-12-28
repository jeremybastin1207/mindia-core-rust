use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use super::{PipelineStepFactory, ScalerFactory, WatermarkerFactory};
use crate::pipeline::PipelineStep;
use crate::storage::FileStorage;
use crate::task::UploadMediaContext;
use crate::transform::{TransformationDescriptorChain, TransformationName};

pub struct PipelineStepsFactory {
    factories: HashMap<TransformationName, Box<dyn PipelineStepFactory>>,
}

impl PipelineStepsFactory {
    pub fn new(file_storage: Arc<Mutex<dyn FileStorage>>) -> Self {
        let mut factories: HashMap<TransformationName, Box<dyn PipelineStepFactory>> =
            HashMap::new();

        factories.insert(
            TransformationName::Scale,
            Box::new(ScalerFactory::default()),
        );

        factories.insert(
            TransformationName::Watermark,
            Box::new(WatermarkerFactory::new(file_storage)),
        );

        Self { factories }
    }

    pub fn register(
        &mut self,
        transformation_name: TransformationName,
        factory: Box<dyn PipelineStepFactory>,
    ) {
        self.factories.insert(transformation_name, factory);
    }

    pub fn create(
        &self,
        transformation_descriptor_chain: TransformationDescriptorChain,
    ) -> Result<Vec<Box<dyn PipelineStep<UploadMediaContext>>>, Box<dyn Error>> {
        let mut pipeline_steps: Vec<Box<dyn PipelineStep<UploadMediaContext>>> = vec![];

        for transformation_descriptor in transformation_descriptor_chain.iter() {
            let pipeline_step = self
                .factories
                .get(transformation_descriptor.name())
                .ok_or("Factory not found")?
                .create(transformation_descriptor.clone())?;

            pipeline_steps.push(pipeline_step);
        }

        Ok(pipeline_steps)
    }
}
