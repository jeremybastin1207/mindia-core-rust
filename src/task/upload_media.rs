use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use crate::extractor::{
    ContextExtractor, ExifExtractor, ExtractorExecutor, NamedTransformationExtractor,
};
use crate::media::Path;
use crate::metadata::MetadataStorage;
use crate::named_transformation::NamedTransformationStorage;
use crate::storage::FileStorage;
use crate::transform::{ContextTransform, TransformExecutor, WebpConverter};

pub struct UploadMedia {
    file_storage: Arc<Mutex<dyn FileStorage>>,
    metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
    named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
}

impl UploadMedia {
    pub fn new(
        file_storage: Arc<Mutex<dyn FileStorage>>,
        metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
        named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
    ) -> UploadMedia {
        UploadMedia {
            file_storage,
            metadata_storage,
            named_transformation_storage,
        }
    }

    pub fn upload(
        &self,
        path: Path,
        transformations: String,
        body: BytesMut,
    ) -> Result<(), Box<dyn Error>> {
        let output = ExtractorExecutor::new().extract(
            ContextExtractor::new(transformations, path.clone(), body.clone().freeze()),
            vec![
                Box::new(ExifExtractor::new()),
                Box::new(NamedTransformationExtractor::new(
                    self.named_transformation_storage.clone(),
                )),
            ],
        )?;

        let context = TransformExecutor::new().transform(
            ContextTransform::new(path.clone(), body.clone()),
            vec![Box::new(WebpConverter::new())],
        )?;

        let path_str = context.path.as_str()?;

        self.file_storage
            .lock()
            .unwrap()
            .upload(path_str, context.body.into())?;

        self.metadata_storage
            .lock()
            .unwrap()
            .save(path_str, output.metadata)?;

        Ok(())
    }
}
