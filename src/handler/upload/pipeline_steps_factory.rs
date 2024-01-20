use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use crate::handler::upload::colorizer_factory::ColorizerFactory;
use crate::handler::upload::watermarker_factory::WatermarkerFactory;

use super::{PipelineStepFactory, ScalerFactory, UploadMediaContext};
use crate::pipeline::PipelineStep;
use crate::storage::FileStorage;
use crate::transform::{TransformationDescriptorChain, TransformationName};

pub struct PipelineStepsFactory {
    factories: Arc<Mutex<HashMap<TransformationName, Box<dyn PipelineStepFactory>>>>,
}

impl PipelineStepsFactory {
    pub fn new(
        file_storage: Arc<dyn FileStorage>,
    ) -> Self {
        let factories: Arc<Mutex<HashMap<TransformationName, Box<dyn PipelineStepFactory>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        factories.lock().unwrap().insert(
            TransformationName::Scale,
            Box::new(ScalerFactory::default()),
        );
        factories.lock().unwrap().insert(
            TransformationName::Watermark,
            Box::new(WatermarkerFactory::new(file_storage.clone())),
        );
        factories.lock().unwrap().insert(
            TransformationName::Colorize,
            Box::new(ColorizerFactory::new(file_storage)),
        );

        Self { factories }
    }

    pub fn register(
        &mut self,
        transformation_name: TransformationName,
        factory: Box<dyn PipelineStepFactory>,
    ) {
        self.factories
            .lock()
            .unwrap()
            .insert(transformation_name, factory);
    }

    pub fn create(
        &self,
        transformation_descriptor_chain: TransformationDescriptorChain,
    ) -> Result<Vec<Box<dyn PipelineStep<UploadMediaContext>>>, Box<dyn Error>> {
        let mut pipeline_steps: Vec<Box<dyn PipelineStep<UploadMediaContext>>> = vec![];

        for transformation_descriptor in transformation_descriptor_chain.iter() {
            let pipeline_step = self
                .factories
                .lock()
                .unwrap()
                .get(transformation_descriptor.name())
                .ok_or("Factory not found")?
                .create(transformation_descriptor.clone())?;

            pipeline_steps.push(pipeline_step);
        }

        Ok(pipeline_steps)
    }
}
