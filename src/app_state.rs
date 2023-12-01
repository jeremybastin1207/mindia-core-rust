use std::sync::Arc;
use std::sync::Mutex;

use crate::apikey_storage::ApiKeyStorage;

pub struct AppState {
    pub apikey_storage: Arc<Mutex<dyn ApiKeyStorage>>,
}
