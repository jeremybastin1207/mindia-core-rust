/* use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::sync::{Arc, Mutex};

use crate::apikey::ApiKeyStorage;

pub struct ApiKeyMiddlewareFactory {
    api_key_storage: Arc<Mutex<dyn ApiKeyStorage>>,
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddleware {
            service,
            api_key_storage: self.api_key_storage.clone(),
        }))
    }
}

pub struct ApiKeyMiddleware<S> {
    service: S,
    api_key_storage: Arc<Mutex<dyn ApiKeyStorage>>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    // Check apikey from header Auhtorization and compare it with apikeys from storage
    fn call(&self, req: ServiceRequest) -> Self::Future {
        Box::pin(async move {
            let api_key_from_req = req
                .headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.strip_prefix("Bearer "))
                .map(|value| value.to_string());

            match api_key_from_req {
                Some(api_key_from_req) => {
                    let api_key_storage = self.api_key_storage.lock().unwrap();
                    let api_key = api_key_storage.get_by_key(&api_key_from_req).unwrap();

                    match api_key {
                        Some(_) => Ok(self.service.call(req).await?),
                        None => Ok(req.into_response(
                            HttpResponse::Unauthorized()
                                .json("Invalid API key")
                                .into_body(),
                        )),
                    }
                }
                None => Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json("API key is missing")
                        .into_body(),
                )),
            }
        })
    }
}
 */
