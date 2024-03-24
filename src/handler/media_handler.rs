use bytes::{Bytes, BytesMut};
use std::{error::Error, sync::Arc};
use crate::extractor::{ContentInfoExtractor, ExifExtractor};
use crate::handler::upload::{PipelineStepsFactory, UploadMediaContext};
use crate::media::{MediaGroupHandle, MediaHandle, Path};
use crate::metadata::{Metadata, MetadataStorage};
use crate::pipeline::PipelineStep;
use crate::storage::FileStorage;
use crate::transform::{PathGenerator, TransformationDescriptorChain, WebpConverter};

#[derive(Clone)]
pub struct MediaHandler {
    file_storage: Arc<dyn FileStorage>,
    cache_storage: Arc<dyn FileStorage>,
    metadata_storage: Arc<dyn MetadataStorage>,
   // pipeline_steps_factory: PipelineStepsFactory,
}

impl MediaHandler {
    pub fn new(
        file_storage: Arc<dyn FileStorage>,
        cache_storage: Arc<dyn FileStorage>,
        metadata_storage: Arc<dyn MetadataStorage>,
    ) -> Self {
        Self {
            file_storage: file_storage.clone(),
            cache_storage: cache_storage.clone(),
            metadata_storage: metadata_storage.clone(),
            //pipeline_steps_factory: PipelineStepsFactory::new(file_storage),
        }
    }

    pub fn read(&self, path: Path) -> Result<Option<Metadata>, Box<dyn Error>> {
        let result = self.metadata_storage.get_by_path(path.as_str())?;

        Ok(result)
    }

    pub async fn upload(
        &self,
        path: Path,
        transformation_chains: Vec<TransformationDescriptorChain>,
        body: BytesMut,
    ) -> Result<Metadata, Box<dyn Error>> {
        let transforms: Vec<Box<dyn PipelineStep<UploadMediaContext>>> = vec![
            Box::new(ExifExtractor::default()),
            Box::new(WebpConverter::default()),
            Box::new(PathGenerator::default()),
            Box::new(ContentInfoExtractor::default()),
        ];

        let mut context = UploadMediaContext::default();
        context.media_handle = MediaHandle::new(body.clone(), Metadata::new(path.clone()));

        for step in transforms {
            context = step.execute(context).await?;
        }

        self.file_storage.upload(
            context.media_handle.metadata.path.as_str(),
            context.media_handle.body.clone().into(),
        ).await?;

        self.metadata_storage.save(
            context.media_handle.metadata.path.as_str(),
            context.media_handle.metadata.clone(),
        )?;

        let mut media_group_handle =
            MediaGroupHandle::new(context.media_handle.clone(), vec![]);

        // if !transformation_chains.is_empty() {
        //     for transformation_chain in transformation_chains {
        //         let mut transformation_steps = self
        //             .pipeline_steps_factory
        //             .create(transformation_chain.clone())?;
        //         transformation_steps.push(Box::new(PathGenerator::default()));
        //         transformation_steps.push(Box::new(ContentInfoExtractor::default()));
        //
        //         let mut sub_context = context.clone();
        //         sub_context.media_handle.metadata.embedded_metadata.clear();
        //         sub_context.transformations = transformation_chain;
        //
        //         for step in transformation_steps {
        //             sub_context = step.execute(sub_context).await?;
        //         }
        //
        //          self.cache_storage.upload(
        //              sub_context.media_handle.metadata.path.as_str(),
        //              sub_context.media_handle.body.clone().into(),
        //          ).await?;
        //
        //         media_group_handle.add_derived_media(sub_context.media_handle);
        //     }
        //
        //     self.metadata_storage.save(
        //         media_group_handle.media.metadata.path.as_str(),
        //         media_group_handle.media.metadata.clone(),
        //     )?;
        // }

        Ok(media_group_handle.media.metadata)
    }

    // pub async fn download(
    //     &self,
    //     path: Path,
    //     transformation_chain: Option<TransformationDescriptorChain>,
    // ) -> Result<Option<Bytes>, Box<dyn Error>> {
    //     if transformation_chain.is_none() {
    //         let body = self.file_storage.download(path.as_str()).await?;
    //         return Ok(body);
    //     }
    //
    //     let transformation_chain = transformation_chain.unwrap();
    //
    //     let derived_path =
    //         PathGenerator::default().transform(&path, &transformation_chain)?;
    //
    //     match self.cache_storage.download(derived_path.as_str()).await? {
    //         Some(body) => Ok(Some(body)),
    //         None => {
    //         //     let mut transformation_steps = self
    //         //         .pipeline_steps_factory
    //         //         .create(transformation_chain.clone())?;
    //         //     transformation_steps.push(Box::new(PathGenerator::default()));
    //         //     transformation_steps.push(Box::new(ContentInfoExtractor::default()));
    //         //
    //         //     match self.file_storage.download(path.as_str()).await? {
    //         //         None => Ok(None),
    //         //         Some(body) => {
    //         //             let mut context = UploadMediaContext::default();
    //         //             context.media_handle = MediaHandle::new(BytesMut::from(&body[..]), Metadata::new(path.clone()));
    //         //
    //         //             for step in transformation_steps {
    //         //                 context = step.execute(context).await?;
    //         //             }
    //         //
    //         //             self.cache_storage.upload(
    //         //                 context.media_handle.metadata.path.as_str(),
    //         //                 context.media_handle.body.clone().into(),
    //         //             ).await?;
    //         //
    //         //             self.metadata_storage.save(
    //         //                 context.media_handle.metadata.path.as_str(),
    //         //                 context.media_handle.metadata.clone()
    //         //             )?;
    //         //
    //         //             Ok(Some(context.media_handle.body.freeze()))
    //         //         }
    //         //     }
    //         // }
    //     }
    // }

    pub async fn move_(&self, src: Path, dst: Path) -> Result<(), Box<dyn Error>> {
        let mut metadata = match self.metadata_storage.get_by_path(src.as_str())? {
            Some(metadata) => metadata,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Metadata not found",
                )));
            }
        };

        let mut new_derived_medias: Vec<Metadata> = Vec::new();

        for mut derived_media in metadata.derived_medias {
            self.cache_storage.move_(derived_media.path.as_str(), dst.as_str()).await?;
            derived_media.path = dst.clone().into();
            new_derived_medias.push(derived_media);
        }

        metadata.derived_medias = new_derived_medias;

        self.file_storage.move_(src.as_str(), dst.as_str()).await?;
        metadata.path = dst.clone().into();

        self.metadata_storage.save(dst.as_str(), metadata.clone())?;

        Ok(())
    }

    pub async fn copy(&self, src: Path, dst: Path) -> Result<(), Box<dyn Error>> {
        let mut metadata = match self.metadata_storage.get_by_path(src.as_str())? {
            Some(metadata) => metadata,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Metadata not found",
                )));
            }
        };

        let mut new_derived_medias: Vec<Metadata> = Vec::new();

        for mut derived_media in metadata.derived_medias {
            self.cache_storage.copy(derived_media.path.as_str(), dst.as_str()).await?;
            derived_media.path = dst.clone().into();
            new_derived_medias.push(derived_media);
        }

        metadata.derived_medias = new_derived_medias;

        self.file_storage.copy(src.as_str(), dst.as_str()).await?;
        metadata.path = dst.clone().into();

        self.metadata_storage.save(dst.as_str(), metadata.clone())?;

        Ok(())
    }

    pub async fn delete(&self, path: Path) -> Result<(), Box<dyn Error>> {
        self.file_storage.delete(path.as_str()).await?;
        self.cache_storage.delete(path.as_str()).await?;
        self.metadata_storage.delete(path.as_str())?;

        Ok(())
    }
}
