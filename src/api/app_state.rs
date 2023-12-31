use std::sync::Arc;

use crate::apikey::ApiKeyStorage;
use crate::config::Config;
use crate::named_transformation::NamedTransformationStorage;
use crate::scheduler::TaskScheduler;
use crate::task::{DeleteMedia, DownloadMedia, ReadMedia, UploadMedia};
use crate::transform::TransformationTemplateRegistry;

pub struct AppState {
    pub apikey_storage: Arc<dyn ApiKeyStorage>,
    pub named_transformation_storage: Arc<dyn NamedTransformationStorage>,
    pub transformation_template_registry: Arc<TransformationTemplateRegistry>,
    pub upload_media: Arc<UploadMedia>,
    pub read_media: Arc<ReadMedia>,
    pub download_media: Arc<DownloadMedia>,
    pub delete_media: Arc<DeleteMedia>,
    pub task_scheduler: Arc<TaskScheduler>,
    pub config: Arc<Config>,
}
