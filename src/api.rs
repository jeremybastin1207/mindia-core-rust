use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;
use std::sync::Mutex;

use crate::apikey::ApiKey;
use crate::apikey_storage::ApiKeyStorage;

pub struct AppState {
    pub apikey_storage: Arc<Mutex<dyn ApiKeyStorage>>,
}

#[get("/apikeys")]
pub async fn get_apikeys(data: web::Data<AppState>) -> impl Responder {
    let mut apikey_storage = data.apikey_storage.lock().unwrap();
    match apikey_storage.get_all() {
        Ok(api_keys) => {
            let api_keys: Vec<ApiKey> = api_keys.into_iter().map(|(_, v)| v).collect();
            HttpResponse::Ok().json(api_keys)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
