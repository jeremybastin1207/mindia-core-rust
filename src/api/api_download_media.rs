use actix_web::{get, web, HttpResponse};
use std::error::Error;

use super::{parse_transformation_from_path, AppState};
use crate::extractor::TransformationsExtractor;
use crate::media::Path;

#[get("/download/{path:.*}")]
pub async fn download_media(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let (transformation_chain_str, path) = parse_transformation_from_path(path.as_str());

    let transformation_chain = if transformation_chain_str.is_empty() {
        None
    } else {
        let named_transformation_storage = data.named_transformation_storage.clone();
        let transformation_template_registry = data.transformation_template_registry.clone();

        let transformation_chain = TransformationsExtractor::new(
            named_transformation_storage,
            transformation_template_registry,
        )
        .extract_one(transformation_chain_str.as_str())?;

        Some(transformation_chain)
    };

    let result = data
        .download_media
        .download(Path::new("/".to_owned() + &path)?, transformation_chain);

    match result {
        Ok(Some(body)) => Ok(HttpResponse::Ok().body(body)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => Err(e),
    }
}
