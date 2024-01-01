use actix_web::{get, web, HttpResponse};
use std::error::Error;

use crate::api::app_state::AppState;
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
