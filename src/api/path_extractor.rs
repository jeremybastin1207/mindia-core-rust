use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;

use crate::api::app_state::AppState;
use crate::api::transformation_chain_extractor::TransformationChainExtractor;
use crate::media::Path;

pub(crate) struct PathExtractor(pub Path);

#[async_trait]
impl FromRequestParts<AppState> for PathExtractor
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let transformation_chain = TransformationChainExtractor::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let path = Path::new(
            parts.uri.path().trim_start_matches(&transformation_chain.transformation_chain_str)
        ).map_err(|_| StatusCode::BAD_REQUEST)?;

        Ok(PathExtractor(path))
    }
}