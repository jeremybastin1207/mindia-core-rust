use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use crate::{
    extractor::TransformationsExtractor,
    transform::TransformationDescriptorChain,
};

pub(crate) struct TransformationChainExtractor {
    pub transformation_chain: Option<TransformationDescriptorChain>,
    pub transformation_chain_str: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for TransformationChainExtractor
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(_parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            transformation_chain: None,
            transformation_chain_str: "".to_string(),
        })
    }
}

// impl FromRequest for TransformationChainExtractor {
//     type Error = actix_web::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
//
//     fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
//         let data_future = web::Data::<AppState>::from_request(req, payload);
//         let path_future = web::Path::<String>::from_request(req, payload);
//
//         Box::pin(async move {
//             let data = data_future.await?;
//             let path = path_future.await?;
//
//             let (transformation_chain_str, _) = parse_transformation_from_path(path.as_str());
//
//             let transformation_chain = if transformation_chain_str.is_empty() {
//                 None
//             } else {
//                 let named_transformation_storage = data.named_transformation_storage.clone();
//                 let transformation_template_registry =
//                     data.transformation_template_registry.clone();
//
//                 match TransformationsExtractor::new(
//                     named_transformation_storage,
//                     transformation_template_registry,
//                 )
//                     .extract_one(transformation_chain_str.as_str())
//                 {
//                     Ok(transformation_chain) => Some(transformation_chain),
//                     Err(e) => return Err(actix_web::Error::from(e)),
//                 }
//             };
//
//             Ok(TransformationChainExtractor {
//                 transformation_chain,
//                 transformation_chain_str,
//             })
//         })
//     }
// }
