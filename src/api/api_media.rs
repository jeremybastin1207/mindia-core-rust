use bytes::BytesMut;
use futures::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use log::error;
use crate::api::app_state::AppState;
use crate::api::path_extractor::PathExtractor;
use crate::api::transformation_chain_extractor::TransformationChainExtractor;

use crate::extractor::TransformationsExtractor;
use crate::media::path::generate_path;
use crate::transform::TransformationDescriptorChain;

// pub async fn read_media(
//     State(state): State<AppState>,
//     Path(path): Path<String>,
// ) -> impl IntoResponse {
//     let path_str = "/".to_owned() + path.as_str();
//     let path = crate::media::Path::new(path_str.as_str()).unwrap();
//
//     match state.media_handler.read(path) {
//         Ok(Some(metadata)) => (StatusCode::OK, serde_json::to_string_pretty(&metadata).unwrap()),
//         Ok(None) => (StatusCode::NOT_FOUND, "Not found".to_string()),
//         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
//     }
// }
//
// pub async fn download_media(
//     State(state): State<AppState>,
//     transformation_chain_extractor: TransformationChainExtractor,
//     path_extractor: PathExtractor,
// ) -> impl IntoResponse {
//     let path = match path_extractor.path {
//         Some(path) => path,
//         None => {
//             error!("Path is not provided");
//             return Err("Path is not provided".into());
//         }
//     };
//
//     let result = match state
//         .media_handler
//         .download(path, transformation_chain_extractor.transformation_chain).await
//     {
//         Ok(result) => result,
//         Err(e) => {
//             error!("Failed to download media: {}", e);
//             return Err(e.into());
//         }
//     };
//
//     match result {
//         Some(body) => Ok((StatusCode::OK, body)),
//         None => {
//             log::warn!("Media not found");
//             Err((StatusCode::NOT_FOUND, "Not found".to_string()))
//         },
//     }
// }
//
// pub async fn upload(
//     State(state): State<AppState>,
//     Path(path): Path<String>,
//     mut payload: Multipart,
// ) -> impl IntoResponse {
//     let named_transformation_storage = state.named_transformation_storage.clone();
//     let transformation_template_registry = state.transformation_template_registry.clone();
//
//     let mut filename = String::new();
//     let mut filedata = BytesMut::new();
//     let mut transformation_chains: Vec<TransformationDescriptorChain> = Vec::new();
//
//     while let Some(item) = payload.next_field().await {
//         let mut field = item?;
//         let content_disposition = field.content_disposition();
//         let field_name = content_disposition.get_name().unwrap();
//
//         if field_name == "file" {
//             filename = content_disposition.get_filename().unwrap().to_string();
//
//             while let Some(chunk) = field.next().await {
//                 let data = chunk?;
//                 filedata.extend_from_slice(&data);
//             }
//         } else if field_name == "transformations" {
//             let mut transformations_str = String::new();
//
//             while let Some(chunk) = field.next().await {
//                 let data = chunk?;
//                 transformations_str.push_str(std::str::from_utf8(&data)?);
//             }
//
//             let transformations_json: Value = serde_json::from_str(&transformations_str)?;
//
//             transformation_chains = if let Some(array) = transformations_json.as_array() {
//                 let str_array: Vec<&str> = array
//                     .iter()
//                     .map(|v| v.as_str().unwrap_or_default())
//                     .collect();
//                 TransformationsExtractor::new(
//                     named_transformation_storage.clone(),
//                     transformation_template_registry.clone(),
//                 )
//                     .extract(str_array)?
//             } else {
//                 return Err("Invalid transformations JSON".into());
//             };
//         }
//     }
//
//     let path = generate_path(format!("{}/{}", path, filename).as_str())?;
//
//     let metadata = state
//         .media_handler
//         .upload(path, transformation_chains, filedata).await?;
//
//     let json_response = serde_json::to_string_pretty(&metadata)?;
//
//     (StatusCode::OK, json_response)
// }
//
// #[derive(Deserialize)]
// struct MoveMediaBody {
//     src: String,
//     dst: String,
// }
//
// pub async fn move_media(
//     State(state): State<AppState>,
//     body: Json<MoveMediaBody>,
// ) -> impl IntoResponse {
//     match state
//         .media_handler
//         .move_(body.src.clone().into(), body.dst.clone().into()).await
//     {
//         Ok(()) => (StatusCode::OK, "Moved"),
//         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
//     }
// }
//
// #[derive(Deserialize)]
// struct CopyMediaBody {
//     src: String,
//     dst: String,
// }
//
// pub async fn copy_media(
//     State(state): State<AppState>,
//     body: Json<CopyMediaBody>,
// ) -> impl IntoResponse {
//     match state
//         .media_handler
//         .copy(body.src.clone().into(), body.dst.clone().into()).await
//     {
//         Ok(()) => (StatusCode::OK, "Copied"),
//         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
//     }
// }

pub async fn delete_media(
    State(state): State<AppState>,
    path_extractor: PathExtractor,
) -> impl IntoResponse {
    let path = path_extractor.path.unwrap();

    match state.media_handler.delete(path).await {
        Ok(()) => (StatusCode::OK, "Deleted").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}
