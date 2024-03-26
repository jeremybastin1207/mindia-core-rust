use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use crate::{
    extractor::TransformationsExtractor,
    transform::TransformationDescriptorChain,
};
use crate::api::app_state::AppState;
use crate::api::utils::parse_transformation_from_path;

pub(crate) struct TransformationChainExtractor {
    pub transformation_chain: Option<TransformationDescriptorChain>,
    pub transformation_chain_str: String,
}

#[async_trait]
impl FromRequestParts<AppState> for TransformationChainExtractor
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let path = parts.uri.path();

        let (transformation_chain_str, _) = parse_transformation_from_path(path);

        let transformation_chain = if transformation_chain_str.is_empty() {
             None
         } else {
             let named_transformation_storage = state.named_transformation_storage.clone();
             let transformation_template_registry =
                 state.transformation_template_registry.clone();

             match TransformationsExtractor::new(
                 named_transformation_storage,
                 transformation_template_registry,
             )
                 .extract_one(transformation_chain_str.as_str())
             {
                 Ok(transformation_chain) => Some(transformation_chain),
                 Err(e) => return Err(()),
             }
         };

         Ok(TransformationChainExtractor {
            transformation_chain,
             transformation_chain_str,
         })
    }
}
