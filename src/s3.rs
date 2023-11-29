use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3 as s3;
use aws_types::region::Region;
use std::collections::HashMap;
use std::error::Error;

pub struct S3Object {
    body: Bytes,
    content_type: Option<String>,
    content_length: Option<i64>,
}

pub struct S3 {
    client: s3::Client,
}

impl S3 {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let region_provider =
            RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));
        let region = region_provider.region().unwrap();
        let client = s3::Client::new(&region);
        Ok(Self { client })
    }

    pub async fn upload_with_multipart(
        &self,
        bucket: String,
        key: String,
        file_bytes: Bytes,
        part_size: usize,
    ) -> Result<(), Box<dyn Error>> {
        let mut part_number = 1;
        let mut completed_parts = HashMap::new();

        let create_multipart_upload_resp = self
            .client
            .create_multipart_upload()
            .bucket(bucket.clone())
            .key(key.clone())
            .send()
            .await?;

        let upload_id = create_multipart_upload_resp.upload_id.unwrap();

        for chunk in file_bytes.chunks(part_size) {
            let upload_part_resp = self
                .client
                .upload_part()
                .bucket(bucket.clone())
                .key(key.clone())
                .part_number(part_number as i32)
                .upload_id(upload_id.clone())
                .body(chunk.to_vec().into())
                .send()
                .await?;

            completed_parts.insert(part_number, upload_part_resp.e_tag.unwrap());
            part_number += 1;
        }

        self.complete_multipart_upload(bucket, key, upload_id, completed_parts)
            .await?;

        Ok(())
    }

    pub async fn download_object(
        &self,
        bucket: String,
        key: String,
    ) -> Result<S3Object, Box<dyn Error>> {
        let resp = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        let body = resp.body.collect().await?;
        let content_type = resp.content_type;
        let content_length = resp.content_length;

        Ok(S3Object {
            body,
            content_type,
            content_length,
        })
    }
}
