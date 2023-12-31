use actix_web::{delete, web, HttpResponse};
use std::error::Error;

use crate::api::app_state::AppState;
use crate::media::Path;

#[delete("/media/{path}")]
pub async fn delete_media(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let path_str = "/".to_owned() + path.as_str();
    let path = Path::new(path_str.as_str())?;

    match data.delete_media.delete(path) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}
