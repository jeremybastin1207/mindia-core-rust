use std::future::Future;
use std::pin::Pin;
use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use futures::executor::block_on;
use futures::future::{ready, Ready};

use super::TransformationChainExtractor;
use crate::media::Path;

pub struct PathExtractor {
    pub path: Option<Path>,
}

impl FromRequest for PathExtractor {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let path_future = web::Path::<String>::from_request(req, payload);
        let transformation_chain_future = TransformationChainExtractor::from_request(req, payload);

        Box::pin(async move {
            let path = path_future.await?;
            let transformation_chain = transformation_chain_future.await?;

            let transformation_chain_str = transformation_chain.transformation_chain_str;
            let path = Path::new(path.as_str().trim_start_matches(&transformation_chain_str)).unwrap();

            Ok(PathExtractor { path: Some(path) })
        })
    }
}
