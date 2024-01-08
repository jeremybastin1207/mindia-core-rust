use std::sync::Arc;

use crate::{
    apikey::ApiKeyStorage,
    config::Config,
    handler::{CacheHandler, MediaHandler},
    scheduler::TaskScheduler,
    transform::{NamedTransformationStorage, TransformationTemplateRegistry},
};

pub struct AppState {
    pub apikey_storage: Arc<dyn ApiKeyStorage>,
    pub named_transformation_storage: Arc<dyn NamedTransformationStorage>,
    pub transformation_template_registry: Arc<TransformationTemplateRegistry>,
    pub media_handler: MediaHandler,
    pub cache_handler: CacheHandler,
    pub task_scheduler: Arc<TaskScheduler>,
    pub config: Config,
}
