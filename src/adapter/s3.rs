use std::collections::HashMap;
use actix_web::web::Bytes;
use aws_sdk_s3 as s3;
use std::error::Error;

pub struct S3Object {
    pub body: Bytes,
    pub content_type: Option<String>,
    pub content_length: Option<i64>,
}

#[derive(Clone)]
pub struct S3 {
    client: s3::Client,
    bucket_name: String,
}

impl S3 {
    pub fn new(client: s3::Client, bucket_name: String) -> Self {
        Self {
            client,
            bucket_name,
        }
    }

    pub async fn upload_object(
        &self,
        key: &str,
        file_bytes: Bytes,
    ) -> Result<(), Box<dyn Error>> {
        let mut part_number = 1;
        let mut completed_parts = HashMap::new();

        let create_multipart_upload_resp = self
            .client
            .create_multipart_upload()
            .bucket(self.bucket_name.as_str())
            .key(key)
            .send()
            .await?;

        let upload_id = create_multipart_upload_resp.upload_id.unwrap();

        for chunk in file_bytes.chunks(4096) {
            let upload_part_resp = self
                .client
                .upload_part()
                .bucket(self.bucket_name.as_str())
                .key(key)
                .part_number(part_number as i32)
                .upload_id(upload_id.clone())
                .body(chunk.to_vec().into())
                .send()
                .await?;

            completed_parts.insert(part_number, upload_part_resp.e_tag.unwrap());
            part_number += 1;
        }

        self.client.complete_multipart_upload();

        Ok(())
    }

    pub async fn download_object(&self, key: &str) -> Result<S3Object, Box<dyn Error>> {
        let resp = self
            .client
            .get_object()
            .bucket(self.bucket_name.as_str())
            .key(key)
            .send()
            .await?;

        let body = resp.body.collect().await?.into_bytes();
        let content_type = resp.content_type;
        let content_length = resp.content_length;

        Ok(S3Object {
            body,
            content_type,
            content_length,
        })
    }

    pub async fn move_object(&self, src_key: &str, dst_key: &str) -> Result<(), Box<dyn Error>> {
        self.copy_object(src_key, dst_key).await?;
        self.delete_object(src_key).await?;

        Ok(())
    }

    pub async fn copy_object(&self, src_key: &str, dst_key: &str) -> Result<(), Box<dyn Error>> {
        self.client
            .copy_object()
            .bucket(self.bucket_name.as_str())
            .key(dst_key)
            .copy_source(format!("{}/{}", self.bucket_name.as_str(), src_key))
            .send()
            .await?;

        Ok(())
    }

    pub async fn delete_object(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.client
            .delete_object()
            .bucket(self.bucket_name.as_str())
            .key(key)
            .send()
            .await?;

        Ok(())
    }
}
