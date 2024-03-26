use std::error::Error;

use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use bytes::BytesMut;
use futures::StreamExt;
use log::error;
use serde::Deserialize;
use serde_json::Value;

use crate::api::app_state::AppState;
use crate::api::path_extractor::PathExtractor;
use crate::api::transformation_chain_extractor::TransformationChainExtractor;
use crate::extractor::TransformationsExtractor;
use crate::media::path::generate_path;
use crate::transform::TransformationDescriptorChain;

pub(crate) async fn read_media(
    State(state): State<AppState>,
    PathExtractor(path): PathExtractor,
) -> impl IntoResponse {
    match state.media_handler.read(path) {
        Ok(Some(metadata)) => (
            StatusCode::OK,
            serde_json::to_string_pretty(&metadata).unwrap(),
        ),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub(crate) async fn download_media(
    State(state): State<AppState>,
    transformation_chain_extractor: TransformationChainExtractor,
    PathExtractor(path): PathExtractor,
) -> impl IntoResponse {
    let result = state
        .media_handler
        .download(path, transformation_chain_extractor.transformation_chain)
        .await;

    match result {
        Ok(Some(body)) => (StatusCode::OK, body).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Not found".to_string()).into_response(),
        Err(e) => {
            error!("Error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub(crate) async fn upload(
    State(state): State<AppState>,
    PathExtractor(path): PathExtractor,
    mut payload: Multipart,
) -> impl IntoResponse {
    let named_transformation_storage = state.named_transformation_storage.clone();
    let transformation_template_registry = state.transformation_template_registry.clone();

    let mut filename = String::new();
    let mut filedata = BytesMut::new();
    let mut transformation_chains: Vec<TransformationDescriptorChain> = Vec::new();

    while let Some(mut field) = payload.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();
        let field_data = field.bytes().await.unwrap();

        if field_name == "file" {
            filename = field_name;
            filedata.extend_from_slice(&field_data);
        } else if field_name == "transformations" {
            let mut transformations_str = String::new();

            transformations_str.push_str(std::str::from_utf8(&field_data).unwrap());

            let transformations_json: Value = match serde_json::from_str(&transformations_str) {
                Ok(json) => json,
                Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            };

            transformation_chains = if let Some(array) = transformations_json.as_array() {
                let str_array: Vec<&str> = array
                    .iter()
                    .map(|v| v.as_str().unwrap_or_default())
                    .collect();
                TransformationsExtractor::new(
                    named_transformation_storage.clone(),
                    transformation_template_registry.clone(),
                )
                .extract(str_array)
                .unwrap()
            } else {
                return (StatusCode::BAD_REQUEST, "Invalid transformations").into_response();
            };
        }
    }

    let path = match generate_path(format!("{}/{}", path.as_str(), filename).as_str()) {
        Ok(path) => path,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    let metadata = match state
        .media_handler
        .upload(path, transformation_chains, filedata)
        .await
    {
        Ok(metadata) => metadata,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let json_response = match serde_json::to_string_pretty(&metadata) {
        Ok(json_response) => json_response,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    (StatusCode::OK, json_response).into_response()
}

#[derive(Deserialize)]
struct MoveMediaBody {
    src: String,
    dst: String,
}

pub(crate) async fn move_media(
    State(state): State<AppState>,
    body: Json<MoveMediaBody>,
) -> impl IntoResponse {
    match state
        .media_handler
        .move_(body.src.clone().into(), body.dst.clone().into())
        .await
    {
        Ok(()) => (StatusCode::OK, "Moved").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct CopyMediaBody {
    src: String,
    dst: String,
}

pub(crate) async fn copy_media(
    State(state): State<AppState>,
    body: Json<CopyMediaBody>,
) -> impl IntoResponse {
    match state
        .media_handler
        .copy(body.src.clone().into(), body.dst.clone().into())
        .await
    {
        Ok(()) => (StatusCode::OK, "Copied").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub(crate) async fn delete_media(
    State(state): State<AppState>,
    PathExtractor(path): PathExtractor,
) -> impl IntoResponse {
    match state.media_handler.delete(path).await {
        Ok(()) => (StatusCode::OK, "Deleted").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
