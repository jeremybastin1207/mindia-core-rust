use actix_web::{get, web, HttpResponse};
use std::error::Error;

use super::{AppState, PathExtractor, TransformationChainExtractor};

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
