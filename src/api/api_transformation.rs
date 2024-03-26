use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api::app_state::AppState;
use crate::transform::NamedTransformation;

pub(crate) async fn get_transformation_templates(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let transformation_templates = state.transformation_template_registry.get_all();
    Json(transformation_templates)
}

pub(crate) async fn get_named_transformations(State(state): State<AppState>) -> impl IntoResponse {
    match state.named_transformation_storage.get_all() {
        Ok(named_transformations) => {
            let named_transformations: Vec<NamedTransformation> =
                named_transformations.into_iter().map(|(_, v)| v).collect();
            (StatusCode::OK, Json(named_transformations)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub(crate) async fn save_named_transformation(
    State(state): State<AppState>,
    Json(new_named_transformation): Json<NamedTransformation>,
) -> impl IntoResponse {
    match state
        .named_transformation_storage
        .save(new_named_transformation.clone())
    {
        Ok(()) => (
            StatusCode::OK,
            format!(
                "Named transformation {} saved",
                new_named_transformation.name
            ),
        ),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub(crate) async fn delete_named_transformation(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match state.named_transformation_storage.delete(&name) {
        Ok(()) => (
            StatusCode::OK,
            format!("Named transformation {} deleted", name),
        ),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
