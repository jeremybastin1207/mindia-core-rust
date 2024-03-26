use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::api::app_state::AppState;
use crate::apikey::ApiKey;
use crate::utils::generate_apikey;

pub(crate) async fn get_apikeys(State(state): State<AppState>) -> impl IntoResponse {
    match state.apikey_storage.get_all() {
        Ok(api_keys) => {
            let api_keys: Vec<ApiKey> = api_keys.into_iter().map(|(_, v)| v).collect();
            (StatusCode::OK, Json(api_keys)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
pub struct SaveApiKeyBody {
    name: String,
}

pub(crate) async fn save_apikey(
    State(data): State<AppState>,
    Json(new_apikey): Json<SaveApiKeyBody>,
) -> impl IntoResponse {
    let apikey = ApiKey {
        name: new_apikey.name.clone(),
        key: generate_apikey(),
    };

    match data.apikey_storage.save(apikey) {
        Ok(()) => (StatusCode::OK, format!("API key {} saved", new_apikey.name)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub(crate) async fn delete_apikey(
    State(data): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match data.apikey_storage.delete(&name) {
        Ok(()) => (StatusCode::OK, format!("API key {} deleted", name)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
