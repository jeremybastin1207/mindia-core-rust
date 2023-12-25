use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use crate::extractor::ExifExtractor;
use crate::media::Path;
use crate::metadata::MetadataStorage;
use crate::pipeline::{Pipeline, PipelineExecutor, Sinker, Source};
use crate::storage::FileStorage;
use crate::transform::TransformationDescriptorChain;
use crate::transform::{PathGenerator, TransformationFactory, WebpConverter};

use super::UploadMediaContext;

pub struct UploadMedia {
    file_storage: Arc<Mutex<dyn FileStorage>>,
    cache_storage: Arc<Mutex<dyn FileStorage>>,
    metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
}

impl UploadMedia {
    pub fn new(
        file_storage: Arc<Mutex<dyn FileStorage>>,
        cache_storage: Arc<Mutex<dyn FileStorage>>,
        metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
    ) -> UploadMedia {
        UploadMedia {
            file_storage,
            cache_storage,
            metadata_storage,
        }
    }

    pub fn upload(
        &self,
        path: Path,
        transformation_chains: Vec<TransformationDescriptorChain>,
        body: BytesMut,
    ) -> Result<(), Box<dyn Error>> {
        let file_storage = self.file_storage.clone();
        let metadata_storage = self.metadata_storage.clone();

        let output = PipelineExecutor::new().execute::<UploadMediaContext>(Pipeline::<
            UploadMediaContext,
        >::new(
            Source::new(Box::new(move |mut context| {
                context.attributes.path = path.clone();
                context.attributes.body = body.clone();

                Ok(context)
            })),
            Sinker::new(Box::new(move |context| {
                file_storage.lock().unwrap().upload(
                    context.attributes.path.as_str()?,
                    context.attributes.body.clone().into(),
                )?;

                metadata_storage.lock().unwrap().save(
                    context.attributes.path.as_str()?,
                    context.attributes.metadata.clone(),
                )?;

                Ok(())
            })),
            vec![
                Box::new(ExifExtractor::default()),
                Box::new(WebpConverter::default()),
                Box::new(PathGenerator::default()),
            ],
        ))?;

        for transformation_chain in transformation_chains {
            let cache_storage = self.cache_storage.clone();
            let output = output.clone();

            let mut transformation_steps =
                TransformationFactory::default().build(transformation_chain.clone())?;
            transformation_steps.push(Box::new(PathGenerator::default()));

            PipelineExecutor::new().execute::<UploadMediaContext>(
                Pipeline::<UploadMediaContext>::new(
                    Source::new(Box::new(move |mut context| {
                        context.attributes.path = output.attributes.path.clone();
                        context.attributes.body = output.attributes.body.clone();
                        context
                            .attributes
                            .transformations
                            .set(transformation_chain.get_trasnfomation_descriptors().clone());

                        Ok(context)
                    })),
                    Sinker::new(Box::new(move |context| {
                        cache_storage.lock().unwrap().upload(
                            context.attributes.path.as_str()?,
                            context.attributes.body.clone().into(),
                        )?;

                        Ok(())
                    })),
                    transformation_steps,
                ),
            )?;
        }

        Ok(())
    }
}
