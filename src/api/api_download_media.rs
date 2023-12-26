use actix_web::{get, web, HttpResponse};
use std::error::Error;

use crate::api::app_state::AppState;
use crate::media::Path;

#[get("/download/{path}")]
pub async fn download_media(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let result = data
        .download_media
        .download(Path::new("/".to_owned() + &path.into_inner())?);

    match result {
        Ok(Some(body)) => Ok(HttpResponse::Ok().body(body)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => Err(e),
    }
}
