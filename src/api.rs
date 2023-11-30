use actix_web::middleware::Logger;
use actix_web::{get, web, web::Data, HttpResponse, Responder, Result};
use std::sync::Arc;

use crate::apikey::ApiKey;
use crate::apikey_storage::ApiKeyStorage;

pub struct AppState {
    pub apikey_storage: Arc<dyn ApiKeyStorage>,
}

#[get("/apikeys")]
pub async fn get_apikeys(data: Data<AppState>) -> Result<impl Responder> {
    match data.apikey_storage.get_all() {
        Ok(api_keys) => {
            let api_keys: Vec<ApiKey> = api_keys.into_iter().map(|(_, v)| v).collect();
            Ok(web::Json(api_keys))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
