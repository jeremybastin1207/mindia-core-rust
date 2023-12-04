use actix_multipart::Multipart;
use actix_web::{post, HttpResponse};
use futures::StreamExt;
use sanitize_filename::sanitize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[post("/upload")]
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Box<dyn Error>> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap();

        if field_name == "file" {
            let filename = content_disposition.get_filename().unwrap();
            let filepath = format!("./{}", sanitize(filename));
            let mut file = File::create(filepath)?;

            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file.write_all(&data)?;
            }
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}
