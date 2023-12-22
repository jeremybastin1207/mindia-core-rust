use bytes::BytesMut;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use crate::extractor::ExifExtractor;
use crate::media::Path;
use crate::metadata::MetadataStorage;
use crate::named_transformation::NamedTransformationStorage;
use crate::pipeline::{Pipeline, PipelineExecutor, Sinker, Source};
use crate::storage::FileStorage;
use crate::transform::WebpConverter;

use super::UploadMediaContext;

pub struct UploadMedia {
    file_storage: Arc<Mutex<dyn FileStorage>>,
    cache_storage: Arc<Mutex<dyn FileStorage>>,
    metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
    named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
}

impl UploadMedia {
    pub fn new(
        file_storage: Arc<Mutex<dyn FileStorage>>,
        cache_storage: Arc<Mutex<dyn FileStorage>>,
        metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
        named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
    ) -> UploadMedia {
        UploadMedia {
            file_storage,
            cache_storage,
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
        let file_storage = self.file_storage.clone();
        let metadata_storage = self.metadata_storage.clone();

        PipelineExecutor::new().execute::<UploadMediaContext>(
            Pipeline::<UploadMediaContext>::new(
                Source::new(Box::new(move |mut context| {
                    context.attributes.path = path.clone();
                    context.attributes.body = body.clone();

                    Ok(context)
                })),
                Sinker::new(Box::new(move |context| {
                    println!("sinker");
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
                    Box::new(ExifExtractor::new()),
                    Box::new(WebpConverter::new()),
                ],
            ),
        )?;

        Ok(())
    }
}
