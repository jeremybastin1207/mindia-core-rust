use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{HeaderMap, HeaderValue};
use axum::http::request::Parts;

use crate::api::app_state::AppState;

pub(crate) struct ApiKeyChecker {}

#[async_trait]
impl FromRequestParts<AppState> for ApiKeyChecker {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let api_key_storage = state.apikey_storage.clone();
        let master_key = state.config.master_key.clone();

        let headers = parts.headers.clone();

        match extract_api_key(headers) {
            Some(api_key_from_req) => {
                if api_key_from_req != master_key {
                    let api_key = { api_key_storage.get_by_key(&api_key_from_req) };

                    match api_key {
                        Ok(Some(_)) => (),
                        _ => return Err(()),
                    }
                }
            }
            None => return Err(()),
        }

        Ok(ApiKeyChecker {})
    }
}

fn extract_api_key(headers: HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value: &HeaderValue| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(|value| value.to_string())
}
