use bytes::Bytes;
use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use crate::media::Path;
use crate::metadata::MetadataStorage;
use crate::pipeline::{Pipeline, PipelineExecutor, Sinker, Source};
use crate::storage::FileStorage;
use crate::transform::TransformationDescriptorChain;
use crate::transform::{PathGenerator, TransformationFactory};

use super::UploadMediaContext;

pub struct DownloadMedia {
    file_storage: Arc<Mutex<dyn FileStorage>>,
    cache_storage: Arc<Mutex<dyn FileStorage>>,
    metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
}

impl DownloadMedia {
    pub fn new(
        file_storage: Arc<Mutex<dyn FileStorage>>,
        cache_storage: Arc<Mutex<dyn FileStorage>>,
        metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
    ) -> DownloadMedia {
        DownloadMedia {
            file_storage,
            cache_storage,
            metadata_storage,
        }
    }
    pub fn download(
        &self,
        path: Path,
        transformation_chain: Option<TransformationDescriptorChain>,
    ) -> Result<Option<Bytes>, Box<dyn Error>> {
        if transformation_chain.is_none() {
            let bytes = self.file_storage.lock().unwrap().download(path.as_str()?)?;
            return Ok(bytes);
        }

        let transformation_chain = transformation_chain.unwrap();

        let derived_path =
            PathGenerator::default().transform(path.clone(), transformation_chain.clone())?;

        let file_bytes = self
            .cache_storage
            .lock()
            .unwrap()
            .download(derived_path.as_str()?)?;

        match file_bytes {
            Some(file_bytes) => Ok(Some(file_bytes)),
            None => {
                let mut transformation_steps =
                    TransformationFactory::default().build(transformation_chain.clone())?;
                transformation_steps.push(Box::new(PathGenerator::default()));

                let metadata_result = self
                    .metadata_storage
                    .lock()
                    .unwrap()
                    .get_by_path(path.as_str()?)?;

                let mut metadata = match metadata_result {
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
                        let file_bytes = file_storage.lock().unwrap().download(path.as_str()?)?;

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
                        cache_storage.lock().unwrap().upload(
                            context.attributes.media_handle.metadata.path.as_str()?,
                            context.attributes.media_handle.body.clone().into(),
                        )?;

                        Ok(())
                    })),
                    transformation_steps,
                ))?;

                metadata.append_derived_media(output.attributes.media_handle.metadata.clone());

                self.metadata_storage.lock().unwrap().save(
                    output.attributes.media_handle.metadata.path.as_str()?,
                    output.attributes.media_handle.metadata.clone(),
                )?;

                Ok(Some(output.attributes.media_handle.body.freeze()))
            }
        }
    }
}
