use actix_web::{delete, web, HttpResponse, Responder};

use crate::api::app_state::AppState;

#[delete("/cache/clear")]
pub async fn clear_cache(data: web::Data<AppState>) -> impl Responder {
    let result = data.clear_cache.clear();

    match result {
        Ok(_) => HttpResponse::Ok().body("Cache cleared successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
