use std::sync::Arc;
use crate::apikey::ApiKeyStorage;
use crate::config::Config;
use crate::handler::{CacheHandler, MediaHandler};
use crate::scheduler::TaskScheduler;
use crate::transform::{NamedTransformationStorage, TransformationTemplateRegistry};

#[derive(Clone)]
pub(crate) struct AppState {
    pub apikey_storage: Arc<dyn ApiKeyStorage>,
    pub named_transformation_storage: Arc<dyn NamedTransformationStorage>,
    pub transformation_template_registry: Arc<TransformationTemplateRegistry>,
    pub media_handler: MediaHandler,
    pub cache_handler: CacheHandler,
    pub task_scheduler: Arc<TaskScheduler>,
    pub config: Config,
}
