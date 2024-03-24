// use std::sync::Arc;
// use axum::extract::FromRequestParts;
//
// use crate::apikey::ApiKeyStorage;
//
// pub struct ApiKeyChecker {
//     api_key_storage: Arc<dyn ApiKeyStorage>,
//     master_key: String,
// }
//
// impl ApiKeyChecker {
//     pub fn new(api_key_storage: Arc<dyn ApiKeyStorage>, master_key: String) -> Self {
//         Self {
//             api_key_storage,
//             master_key,
//         }
//     }
// }
//
// impl<S> FromRequestParts<S> for ApiKeyChecker
// where
//     S: Send + Sync,
// {
//     type Rejection = ();
//
//     fn from_request_parts(_parts: &mut axum::http::request::Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         Ok(Self {
//             api_key_storage: Arc::new(crate::apikey::InMemoryApiKeyStorage::new()),
//             master_key: "master".to_string(),
//         })
//     }
// }
//
// // impl<S, B> Transform<S, ServiceRequest> for ApiKeyChecker
// // where
// //     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
// //     S::Future: 'static,
// //     B: 'static,
// // {
// //     type Response = ServiceResponse<EitherBody<B, BoxBody>>;
// //     type Error = Error;
// //     type Transform = ApiKeyCheckerMiddleware<S>;
// //     type InitError = ();
// //     type Future = Ready<Result<Self::Transform, Self::InitError>>;
// //
// //     fn new_transform(&self, service: S) -> Self::Future {
// //         ready(Ok(ApiKeyCheckerMiddleware {
// //             service,
// //             api_key_storage: self.api_key_storage.clone(),
// //             master_key: self.master_key.clone(),
// //         }))
// //     }
// // }
// //
// // pub struct ApiKeyCheckerMiddleware<S> {
// //     service: S,
// //     api_key_storage: Arc<dyn ApiKeyStorage>,
// //     master_key: String,
// // }
// //
// // impl<S, B> Service<ServiceRequest> for ApiKeyCheckerMiddleware<S>
// // where
// //     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
// //     S::Future: 'static,
// //     B: 'static,
// // {
// //     type Response = ServiceResponse<EitherBody<B, BoxBody>>;
// //     type Error = Error;
// //     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
// //
// //     forward_ready!(service);
// //
// //     fn call(&self, req: ServiceRequest) -> Self::Future {
// //         let api_key_from_req = req
// //             .headers()
// //             .get("Authorization")
// //             .and_then(|value| value.to_str().ok())
// //             .and_then(|value| value.strip_prefix("Bearer "))
// //             .map(|value| value.to_string());
// //
// //         match api_key_from_req {
// //             Some(api_key_from_req) => {
// //                 if api_key_from_req != self.master_key {
// //                     let api_key = { self.api_key_storage.get_by_key(&api_key_from_req) };
// //
// //                     match api_key {
// //                         Ok(Some(_)) => (),
// //                         _ => {
// //                             return Box::pin(async move {
// //                                 let res = req
// //                                     .into_response(HttpResponse::Forbidden().finish())
// //                                     .map_into_right_body();
// //                                 Ok(res)
// //                             })
// //                         }
// //                     }
// //                 }
// //             }
// //             None => {
// //                 return Box::pin(async move {
// //                     let res = req
// //                         .into_response(HttpResponse::Forbidden().finish())
// //                         .map_into_right_body();
// //                     Ok(res)
// //                 })
// //             }
// //         }
// //
// //         let fut = self.service.call(req);
// //
// //         Box::pin(async move { Ok(fut.await?.map_into_left_body()) })
// //     }
// // }
