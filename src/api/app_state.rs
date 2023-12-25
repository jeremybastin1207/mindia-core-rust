use crate::apikey::ApiKeyStorage;
use crate::named_transformation::NamedTransformationStorage;
use crate::task::upload_media;
use crate::transform::TransformationTemplateRegistry;
use std::sync::Arc;
use std::sync::Mutex;

pub struct AppState {
    pub apikey_storage: Arc<Mutex<dyn ApiKeyStorage>>,
    pub named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
    pub transformation_template_registry: Arc<Mutex<TransformationTemplateRegistry>>,
    pub upload_media: Arc<upload_media::UploadMedia>,
}
