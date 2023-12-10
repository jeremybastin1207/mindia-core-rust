use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse};
use bytes::BytesMut;
use futures::StreamExt;
use std::error::Error;

use crate::api::app_state::AppState;
use crate::media::Path;

#[post("/upload")]
pub async fn upload(
    data: web::Data<AppState>,
    mut payload: Multipart,
) -> Result<HttpResponse, Box<dyn Error>> {
    let mut metadata = String::new();
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
        } else if field_name == "metadata" {
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                metadata.push_str(std::str::from_utf8(&data)?);
            }
        }
    }

    data.upload_media
        .upload(Path::new("/".to_owned() + &filename)?, filedata)?;

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}
