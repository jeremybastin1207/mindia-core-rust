use std::sync::Arc;
use std::sync::Mutex;

use crate::apikey::ApiKeyStorage;
use crate::named_transformation::NamedTransformationStorage;
//use crate::task::upload_media;

pub struct AppState {
    pub apikey_storage: Arc<Mutex<dyn ApiKeyStorage>>,
    pub named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
    //pub upload_media: upload_media::UploadMedia,
}
