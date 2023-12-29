use actix_web::{delete, web, HttpResponse, Responder};
use tokio::task;

use crate::api::app_state::AppState;

#[delete("/cache/clear")]
pub async fn clear_cache(data: web::Data<AppState>) -> impl Responder {
    let clear_cache = data.clear_cache.clone();

    task::spawn(async move {
        clear_cache.clear().await.unwrap();
    });

    HttpResponse::Ok().body("Clear cache task pushed to background")
}
