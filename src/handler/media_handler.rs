use bytes::Bytes;
use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;

use super::{PipelineStepsFactory, UploadMediaContext};
use crate::extractor::ExifExtractor;
use crate::media::{MediaGroupHandle, Path};
use crate::metadata::{Metadata, MetadataStorage};
use crate::pipeline::{Pipeline, PipelineExecutor, Sinker, Source};
use crate::storage::FileStorage;
use crate::transform::{PathGenerator, TransformationDescriptorChain, WebpConverter};

pub struct MediaHandler {
    file_storage: Arc<dyn FileStorage>,
    cache_storage: Arc<dyn FileStorage>,
    metadata_storage: Arc<dyn MetadataStorage>,
    pipline_steps_factory: PipelineStepsFactory,
}

impl MediaHandler {
    pub fn new(
        file_storage: Arc<dyn FileStorage>,
        cache_storage: Arc<dyn FileStorage>,
        metadata_storage: Arc<dyn MetadataStorage>,
    ) -> Self {
        Self {
            file_storage: file_storage.clone(),
            cache_storage,
            metadata_storage,
            pipline_steps_factory: PipelineStepsFactory::new(file_storage),
        }
    }

    pub fn read(&self, path: Path) -> Result<Option<Metadata>, Box<dyn Error>> {
        let result = self.metadata_storage.get_by_path(path.as_str()?)?;

        Ok(result)
    }

    pub fn upload(
        &self,
        path: Path,
        transformation_chains: Vec<TransformationDescriptorChain>,
        body: BytesMut,
    ) -> Result<Metadata, Box<dyn Error>> {
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
                file_storage.upload(
                    context.attributes.media_handle.metadata.path.as_str()?,
                    context.attributes.media_handle.body.clone().into(),
                )?;

                metadata_storage.save(
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

                let mut transformation_steps = self
                    .pipline_steps_factory
                    .create(transformation_chain.clone())?;
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
                        cache_storage.upload(
                            context.attributes.media_handle.metadata.path.as_str()?,
                            context.attributes.media_handle.body.clone().into(),
                        )?;

                        Ok(())
                    })),
                    transformation_steps,
                ))?;

                media_group_handle.add_derived_media(output.attributes.media_handle);
            }

            metadata_storage.save(
                media_group_handle.media.metadata.path.as_str()?,
                media_group_handle.media.metadata.clone(),
            )?;
        }

        Ok(output.attributes.media_handle.metadata)
    }

    pub fn download(
        &self,
        path: Path,
        transformation_chain: Option<TransformationDescriptorChain>,
    ) -> Result<Option<Bytes>, Box<dyn Error>> {
        if transformation_chain.is_none() {
            let bytes = self.file_storage.download(path.as_str()?)?;
            return Ok(bytes);
        }

        let transformation_chain = transformation_chain.unwrap();

        let derived_path =
            PathGenerator::default().transform(path.clone(), transformation_chain.clone())?;

        let file_bytes = self.cache_storage.download(derived_path.as_str()?)?;

        match file_bytes {
            Some(file_bytes) => Ok(Some(file_bytes)),
            None => {
                let mut transformation_steps = self
                    .pipline_steps_factory
                    .create(transformation_chain.clone())?;
                transformation_steps.push(Box::new(PathGenerator::default()));

                let mut metadata = match self.metadata_storage.get_by_path(path.as_str()?)? {
                    Some(metadata) => metadata,
                    None => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            "Metadata not found",
                        )));
                    }
                };

                let file_storage = self.file_storage.clone();
                let cache_storage = self.cache_storage.clone();

                let output = PipelineExecutor::new().execute::<UploadMediaContext>(Pipeline::<
                    UploadMediaContext,
                >::new(
                    Source::new(Box::new(move |mut context| {
                        let file_bytes = file_storage.download(path.as_str()?)?;

                        if let Some(file_bytes) = file_bytes {
                            context.attributes.media_handle.body = BytesMut::from(&file_bytes[..]);
                        } else {
                            return Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::NotFound,
                                "File not found",
                            )));
                        }

                        context.attributes.media_handle.metadata.path = path.clone();
                        context
                            .attributes
                            .transformations
                            .set(transformation_chain.get_trasnfomation_descriptors().clone());

                        Ok(context)
                    })),
                    Sinker::new(Box::new(move |context| {
                        cache_storage.upload(
                            context.attributes.media_handle.metadata.path.as_str()?,
                            context.attributes.media_handle.body.clone().into(),
                        )?;

                        Ok(())
                    })),
                    transformation_steps,
                ))?;

                metadata.append_derived_media(output.attributes.media_handle.metadata.clone());

                self.metadata_storage
                    .save(metadata.path.as_str()?, metadata.clone())?;

                Ok(Some(output.attributes.media_handle.body.freeze()))
            }
        }
    }

    pub fn delete(&self, path: Path) -> Result<(), Box<dyn Error>> {
        let file_storage = self.file_storage.clone();
        let cache_storage = self.cache_storage.clone();
        let metadata_storage = self.metadata_storage.clone();

        file_storage.delete(path.as_str()?)?;
        cache_storage.delete(path.as_str()?)?;
        metadata_storage.delete(path.as_str()?)?;

        Ok(())
    }
}
