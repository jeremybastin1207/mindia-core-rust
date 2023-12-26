use crate::apikey::ApiKeyStorage;
use crate::named_transformation::NamedTransformationStorage;
use crate::task::{download_media, read_media, upload_media};
use crate::transform::TransformationTemplateRegistry;
use std::sync::Arc;
use std::sync::Mutex;

pub struct AppState {
    pub apikey_storage: Arc<Mutex<dyn ApiKeyStorage>>,
    pub named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
    pub transformation_template_registry: Arc<Mutex<TransformationTemplateRegistry>>,
    pub upload_media: Arc<upload_media::UploadMedia>,
    pub read_media: Arc<read_media::ReadMedia>,
    pub download_media: Arc<download_media::DownloadMedia>,
}
