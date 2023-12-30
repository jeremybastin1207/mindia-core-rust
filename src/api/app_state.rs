use std::sync::Arc;

use crate::apikey::ApiKeyStorage;
use crate::config::Config;
use crate::named_transformation::NamedTransformationStorage;
use crate::task::{clear_cache, download_media, read_media, upload_media};
use crate::transform::TransformationTemplateRegistry;

pub struct AppState {
    pub apikey_storage: Arc<dyn ApiKeyStorage>,
    pub named_transformation_storage: Arc<dyn NamedTransformationStorage>,
    pub transformation_template_registry: Arc<TransformationTemplateRegistry>,
    pub upload_media: Arc<upload_media::UploadMedia>,
    pub read_media: Arc<read_media::ReadMedia>,
    pub download_media: Arc<download_media::DownloadMedia>,
    pub clear_cache: Arc<clear_cache::ClearCache>,
    pub config: Arc<Config>,
}
