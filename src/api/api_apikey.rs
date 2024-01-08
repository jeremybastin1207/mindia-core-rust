use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::{
    api::app_state::AppState,
    apikey::ApiKey,
    utils::generate_apikey,
};

#[get("/apikey")]
pub async fn get_apikeys(data: web::Data<AppState>) -> impl Responder {
    match data.apikey_storage.get_all() {
        Ok(api_keys) => {
            let api_keys: Vec<ApiKey> = api_keys.into_iter().map(|(_, v)| v).collect();
            HttpResponse::Ok().json(api_keys)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/apikey")]
pub async fn save_apikey(
    data: web::Data<AppState>,
    new_apikey: web::Json<ApiKey>,
) -> impl Responder {
    let apikey = ApiKey {
        name: new_apikey.name.clone(),
        key: generate_apikey(),
    };

    match data.apikey_storage.save(apikey) {
        Ok(()) => HttpResponse::Ok().body(format!("API key {} saved", new_apikey.name)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/apikey/{name}")]
pub async fn delete_apikey(data: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    match data.apikey_storage.delete(&name) {
        Ok(()) => HttpResponse::Ok().body(format!("API key {} deleted", name)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
