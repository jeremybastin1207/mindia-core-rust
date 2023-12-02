use std::sync::Arc;
use std::sync::Mutex;

use crate::apikey::ApiKeyStorage;
use crate::named_transformation::NamedTransformationStorage;

pub struct AppState {
    pub apikey_storage: Arc<Mutex<dyn ApiKeyStorage>>,
    pub named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>>,
}
