use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;

use crate::api::app_state::AppState;
use crate::scheduler::{Details, Task, TaskKind};

pub(crate) async fn clear_cache(State(state): State<AppState>) -> impl IntoResponse {
    let task_scheduler = state.task_scheduler.clone();

    let task = Task::new(
        TaskKind::ClearCache,
        Details::ClearCache {
            before_date: Utc::now(),
        },
    );

    match task_scheduler.push(task) {
        Ok(_) => (StatusCode::OK, "Cache task pushed to scheduler").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
