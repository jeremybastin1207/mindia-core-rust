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
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let path_result = block_on(web::Path::<String>::from_request(req, payload));
        let transformation_chain_result =
            block_on(TransformationChainExtractor::from_request(req, payload));

        match (path_result, transformation_chain_result) {
            (Ok(path), Ok(transformation_chain)) => {
                let transformation_chain_str = transformation_chain.transformation_chain_str;

                let path =
                    Path::new(path.as_str().trim_start_matches(&transformation_chain_str)).unwrap();

                ready(Ok(PathExtractor { path: Some(path) }))
            }
            (Err(e), _) | (_, Err(e)) => ready(Err(e.into())),
        }
    }
}
