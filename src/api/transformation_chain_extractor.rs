use actix_web::{web, FromRequest, HttpRequest, dev::Payload};
use futures::{future::{ready, Ready}, executor::block_on};

use super::{parse_transformation_from_path, AppState};
use crate::{
    extractor::TransformationsExtractor,
    transform::TransformationDescriptorChain,
};

pub struct TransformationChainExtractor {
    pub transformation_chain: Option<TransformationDescriptorChain>,
    pub transformation_chain_str: String,
}

impl FromRequest for TransformationChainExtractor {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = block_on(web::Data::<AppState>::from_request(req, &mut Payload::None));
        let path = block_on(web::Path::<String>::from_request(req, &mut Payload::None));

        match (data, path) {
            (Ok(data), Ok(path)) => {
                let (transformation_chain_str, _) = parse_transformation_from_path(path.as_str());

                let transformation_chain = if transformation_chain_str.is_empty() {
                    None
                } else {
                    let named_transformation_storage = data.named_transformation_storage.clone();
                    let transformation_template_registry =
                        data.transformation_template_registry.clone();

                    match TransformationsExtractor::new(
                        named_transformation_storage,
                        transformation_template_registry,
                    )
                    .extract_one(transformation_chain_str.as_str())
                    {
                        Ok(transformation_chain) => Some(transformation_chain),
                        Err(e) => return ready(Err(actix_web::Error::from(e))),
                    }
                };

                ready(Ok(TransformationChainExtractor {
                    transformation_chain,
                    transformation_chain_str,
                }))
            }
            (Err(e), _) | (_, Err(e)) => ready(Err(e.into())),
        }
    }
}
