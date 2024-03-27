use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, routing::get};
use axum::http::{HeaderName, HeaderValue, Method};
use axum::routing::{delete, post};
use tokio::sync::Mutex;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::api::api_apikey::{delete_apikey, get_apikeys, save_apikey};
use crate::api::api_clear_cache::clear_cache;
use crate::api::api_media::{delete_media, read_media, upload_media};
use crate::api::api_transformation::{
    delete_named_transformation, get_named_transformations, get_transformation_templates,
    save_named_transformation,
};
use crate::api::app_state::AppState;
use crate::apikey::ApiKeyStorage;
use crate::config::Config;
use crate::handler::{CacheHandler, MediaHandler};
use crate::metadata::MetadataStorage;
use crate::scheduler::TaskScheduler;
use crate::storage::FileStorage;
use crate::transform::{NamedTransformationStorage, TransformationTemplateRegistry};

pub async fn run_server(
    config: Config,
    file_storage: Arc<Mutex<dyn FileStorage>>,
    cache_storage: Arc<Mutex<dyn FileStorage>>,
    metadata_storage: Arc<Mutex<dyn MetadataStorage>>,
    named_transformation_storage: Arc<dyn NamedTransformationStorage>,
    apikey_storage: Arc<dyn ApiKeyStorage>,
    task_scheduler: Arc<TaskScheduler>,
) -> std::io::Result<()> {
    let shared_state = AppState {
        apikey_storage: apikey_storage.clone(),
        named_transformation_storage: named_transformation_storage.clone(),
        transformation_template_registry: Arc::new(TransformationTemplateRegistry::new()),
        media_handler: MediaHandler::new(
            file_storage.clone(),
            cache_storage.clone(),
            metadata_storage.clone(),
        ),
        cache_handler: CacheHandler::new(cache_storage.clone(), metadata_storage.clone()),
        task_scheduler: task_scheduler.clone(),
        config: config.clone(),
    };

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(AllowOrigin::exact(HeaderValue::from_static(
            "http://localhost:3000",
        )))
        .allow_headers(vec![HeaderName::from_static("content-type")])
        .allow_credentials(true);

    let app = Router::new()
        .route("/", get(|| async { "Mindia API" }))
        .nest(
            "/api/v0",
            Router::new()
                .nest(
                    "/named_transformation",
                    Router::new()
                        .route("/templates", get(get_transformation_templates))
                        .route("/", get(get_named_transformations))
                        .route("/", post(save_named_transformation))
                        .route("/:name", delete(delete_named_transformation)),
                )
                .nest(
                    "/apikey",
                    Router::new()
                        .route("/", get(get_apikeys))
                        .route("/", post(save_apikey))
                        .route("/:name", delete(delete_apikey)),
                )
                .nest(
                    "/media",
                    Router::new()
                        .route("/*path", get(read_media))
                        .route("/upload/*path", post(upload_media))
                        .route("/*path", delete(delete_media)),
                )
                .nest("/cache", Router::new().route("/", post(clear_cache))),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state);

    let bind_address = format!("127.0.0.1:{}", config.server.port.clone());

    let listener = tokio::net::TcpListener::bind(bind_address.clone())
        .await
        .unwrap();

    println!("Server is running at http://{}", bind_address);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}
