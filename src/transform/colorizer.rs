use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::error::Error;
use std::time::Duration;
use actix_rt::time::sleep;
use bytes::BytesMut;
use std::sync::Arc;
use serde::{Deserialize};
use crate::media::Path;
use crate::storage::FileStorage;


#[derive(Debug, PartialEq, Deserialize)]
enum Status {
    Starting,
    Processing,
    Succeeded,
    Failed,
    Canceled,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Starting => "starting",
            Status::Processing => "processing",
            Status::Succeeded => "succeeded",
            Status::Failed => "failed",
            Status::Canceled => "canceled",
        }
    }
}

#[derive(Debug, Deserialize)]
struct PredictionResult {
    id: String,
    status: Status,
    output_url: String,
}

pub struct Colorizer {
    file_storage: Arc<dyn FileStorage>,
}

impl Colorizer {
    pub fn new(
        file_storage: Arc<dyn FileStorage>,
    ) -> Self {
        Self {
            file_storage,
        }
    }

    pub async fn transform(&self, path: Path) -> Result<(Path, BytesMut), Box<dyn Error>> {
        let mut prediction = self.create_prediction().await?;

        while prediction.status == Status::Processing || prediction.status == Status::Starting {
            sleep(Duration::from_millis(5000)).await;
            prediction = self.get_prediction().await?;
        }

        let body = self.download_output(prediction.output_url.as_str()).await?;

        Ok((path, body))
    }

    pub async fn download_output(&self, output_url: &str) -> Result<BytesMut, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let res = client.get(output_url)
            .send()
            .await?;

        let bytes = res.bytes().await?;

        Ok(BytesMut::from(&bytes[..]))
    }

    pub async fn create_prediction(&self) -> Result<PredictionResult, Box<dyn Error>> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static(""));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let body = json!({
            "version": "376c74a2c9eb442a2ff9391b84dc5b949cd4e80b4dc0565115be0a19b7df0ae6",
            "input": {
                "input_image": "",
                "model_name": "Artistic",
		        "render_factor": 35,
            }
        });
        
        let res = client.post("https://api.replicate.com/v1/predictions")
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;
        println!("Body:\n{}", body);

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn get_prediction(&self) -> Result<PredictionResult, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("<paste-your-token-here>"));

        let res = client.get("https://api.replicate.com/v1/predictions/gm3qorzdhgbfurvjtvhg6dckhu")
            .headers(headers)
            .send()
            .await?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;
        println!("Body:\n{}", body);

        Ok(serde_json::from_str(&body)?)
    }
}