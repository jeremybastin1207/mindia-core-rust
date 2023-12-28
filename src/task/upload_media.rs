use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use super::UploadMediaContext;
use crate::extractor::ExifExtractor;
use crate::media::{MediaGroupHandle, Path};
use crate::metadata::MetadataStorage;
use crate::pipeline::{Pipeline, PipelineExecutor, Sinker, Source};
use crate::storage::FileStorage;
use crate::transform::{
    PathGenerator, TransformationDescriptorChain, TransformationFactory, WebpConverter,
};

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
                context.attributes.media_handle.metadata.path = path.clone();
                context.attributes.media_handle.body = body.clone();

                Ok(context)
            })),
            Sinker::new(Box::new(move |context| {
                file_storage.lock().unwrap().upload(
                    context.attributes.media_handle.metadata.path.as_str()?,
                    context.attributes.media_handle.body.clone().into(),
                )?;

                metadata_storage.lock().unwrap().save(
                    context.attributes.media_handle.metadata.path.as_str()?,
                    context.attributes.media_handle.metadata.clone(),
                )?;

                Ok(())
            })),
            vec![
                Box::new(ExifExtractor::default()),
                Box::new(WebpConverter::default()),
                Box::new(PathGenerator::default()),
            ],
        ))?;

        if !transformation_chains.is_empty() {
            let metadata_storage = self.metadata_storage.clone();

            let mut media_group_handle =
                MediaGroupHandle::new(output.attributes.media_handle.clone(), vec![]);

            for transformation_chain in transformation_chains {
                let cache_storage = self.cache_storage.clone();
                let output = output.clone();

                let mut transformation_steps =
                    TransformationFactory::default().build(transformation_chain.clone())?;
                transformation_steps.push(Box::new(PathGenerator::default()));

                let output = PipelineExecutor::new().execute::<UploadMediaContext>(Pipeline::<
                    UploadMediaContext,
                >::new(
                    Source::new(Box::new(move |mut context| {
                        context.attributes.media_handle.metadata.path =
                            output.attributes.media_handle.metadata.path.clone();
                        context.attributes.media_handle.body =
                            output.attributes.media_handle.body.clone();
                        context
                            .attributes
                            .transformations
                            .set(transformation_chain.get_trasnfomation_descriptors().clone());

                        Ok(context)
                    })),
                    Sinker::new(Box::new(move |context| {
                        cache_storage.lock().unwrap().upload(
                            context.attributes.media_handle.metadata.path.as_str()?,
                            context.attributes.media_handle.body.clone().into(),
                        )?;

                        Ok(())
                    })),
                    transformation_steps,
                ))?;

                media_group_handle.add_derived_media(output.attributes.media_handle);
            }

            metadata_storage.lock().unwrap().save(
                media_group_handle.media.metadata.path.as_str()?,
                media_group_handle.media.metadata.clone(),
            )?;
        }

        Ok(())
    }
}
