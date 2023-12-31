use actix_web::{delete, web, HttpResponse, Responder};
use chrono::Utc;

use crate::api::app_state::AppState;
use crate::scheduler::{Details, Task, TaskKind};

#[delete("/cache/clear")]
pub async fn clear_cache(data: web::Data<AppState>) -> impl Responder {
    let task_scheduler = data.task_scheduler.clone();

    match task_scheduler.push(Task::new(
        TaskKind::ClearCache,
        Details::ClearCache {
            before_date: Utc::now(),
        },
    )) {
        Ok(_) => HttpResponse::Ok().body("Cache task pushed to scheduler"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
