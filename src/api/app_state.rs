use crate::apikey::ApiKeyStorage;
use crate::named_transformation::NamedTransformationStorage;
use crate::task::upload_media;
use std::sync::Arc;
use std::sync::Mutex;

pub struct AppState {
    pub apikey_storage: Arc<Mutex<dyn ApiKeyStorage>>,
    pub named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
    pub upload_media: Arc<upload_media::UploadMedia>,
}
