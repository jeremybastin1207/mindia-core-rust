use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse};
use bytes::BytesMut;
use futures::StreamExt;
use serde_json::Value;
use std::error::Error;

use crate::api::app_state::AppState;
use crate::extractor::TransformationsExtractor;
use crate::media::Path;

#[post("/upload/{path:.*}")]
pub async fn upload(
    data: web::Data<AppState>,
    path: web::Path<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, Box<dyn Error>> {
    let named_transformation_storage = data.named_transformation_storage.clone();
    let transformation_template_registry = data.transformation_template_registry.clone();

    let mut transformations_str = String::new();
    let mut filename = String::new();
    let mut filedata = BytesMut::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap();

        if field_name == "file" {
            filename = content_disposition.get_filename().unwrap().to_string();

            while let Some(chunk) = field.next().await {
                let data = chunk?;
                filedata.extend_from_slice(&data);
            }
        } else if field_name == "transformations" {
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                transformations_str.push_str(std::str::from_utf8(&data)?);
            }
        }
    }

    let transformations_json: Value = serde_json::from_str(&transformations_str)?;

    let transformation_chains = if let Some(array) = transformations_json.as_array() {
        let str_array: Vec<&str> = array
            .iter()
            .map(|v| v.as_str().unwrap_or_default())
            .collect();
        TransformationsExtractor::new(
            named_transformation_storage,
            transformation_template_registry,
        )
        .extract(str_array)?
    } else {
        return Err("Invalid transformations JSON".into());
    };

    let path_str = format!("/{}/{}", path, filename);
    let path = Path::generate(&path_str)?;

    let metadata = data
        .upload_media
        .upload(path, transformation_chains, filedata)?;

    Ok(HttpResponse::Ok().body(serde_json::to_string(&metadata)?))
}
