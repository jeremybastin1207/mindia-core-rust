use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, HttpResponse};
use bytes::BytesMut;
use futures::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

use super::{AppState, PathExtractor, TransformationChainExtractor};
use crate::extractor::TransformationsExtractor;
use crate::media::Path;

#[get("/media/{path}")]
pub async fn read_media(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let path_str = "/".to_owned() + path.as_str();
    let path = Path::new(path_str.as_str())?;

    match data.media_handler.read(path) {
        Ok(Some(metadata)) => Ok(HttpResponse::Ok().json(metadata)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => Err(e),
    }
}

#[get("/download/{path:.*}")]
pub async fn download_media(
    data: web::Data<AppState>,
    transformation_chain_extractor: TransformationChainExtractor,
    path_extractor: PathExtractor,
) -> Result<HttpResponse, Box<dyn Error>> {
    let path = path_extractor.path.unwrap();

    let result = data
        .media_handler
        .download(path, transformation_chain_extractor.transformation_chain);

    match result {
        Ok(Some(body)) => Ok(HttpResponse::Ok().body(body)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => Err(e.into()),
    }
}

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
        .media_handler
        .upload(path, transformation_chains, filedata)?;

    Ok(HttpResponse::Ok().body(serde_json::to_string(&metadata)?))
}

#[derive(Deserialize)]
struct MoveMediaBody {
    src: String,
    dst: String,
}

#[post("/media/move")]
pub async fn move_media(
    data: web::Data<AppState>,
    body: web::Json<MoveMediaBody>,
) -> Result<HttpResponse, Box<dyn Error>> {
    match data
        .media_handler
        .move_(body.src.clone().into(), body.dst.clone().into())
    {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct CopyMediaBody {
    src: String,
    dst: String,
}

#[post("/media/copy")]
pub async fn copy_media(
    data: web::Data<AppState>,
    body: web::Json<CopyMediaBody>,
) -> Result<HttpResponse, Box<dyn Error>> {
    match data
        .media_handler
        .copy(body.src.clone().into(), body.dst.clone().into())
    {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}

#[delete("/media/{path}")]
pub async fn delete_media(
    data: web::Data<AppState>,
    path_extractor: PathExtractor,
) -> Result<HttpResponse, Box<dyn Error>> {
    let path = path_extractor.path.unwrap();

    match data.media_handler.delete(path) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}
