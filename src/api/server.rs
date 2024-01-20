use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use crate::api::app_state::AppState;
use crate::api::middleware_apikey::ApiKeyChecker;
use crate::apikey::ApiKeyStorage;
use crate::config::Config;
use crate::handler::{CacheHandler, MediaHandler};
use crate::metadata::MetadataStorage;
use crate::scheduler::TaskScheduler;
use crate::storage::FileStorage;
use crate::transform::{NamedTransformationStorage, TransformationTemplateRegistry};
use crate::api::api_apikey::{delete_apikey, get_apikeys, save_apikey};
use crate::api::api_transformation::{delete_named_transformation, get_named_transformations, get_transformation_templates, save_named_transformation};
use crate::api::api_media::{copy_media, delete_media, download_media, move_media, read_media, upload};
use crate::api::api_clear_cache::clear_cache;


pub async fn run_server(
    config: Config,
    file_storage: Arc<dyn FileStorage>,
    cache_storage: Arc<dyn FileStorage>,
    metadata_storage: Arc<dyn MetadataStorage>,
    named_transformation_storage: Arc<dyn NamedTransformationStorage>,
    apikey_storage: Arc<dyn ApiKeyStorage>,
    task_scheduler: Arc<TaskScheduler>,
) -> std::io::Result<()> {
    let bind_address = format!("127.0.0.1:{}", config.server.port.clone());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_cors::Cors::default())
            .wrap(ApiKeyChecker::new(
                apikey_storage.clone(),
                config.master_key.clone(),
            ))
            .app_data(web::Data::new(AppState {
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
            }))
            .service(
                web::scope("/api/v0")
                    .service(get_apikeys)
                    .service(save_apikey)
                    .service(delete_apikey)
                    .service(get_named_transformations)
                    .service(save_named_transformation)
                    .service(delete_named_transformation)
                    .service(get_transformation_templates)
                    .service(upload)
                    .service(read_media)
                    .service(download_media)
                    .service(clear_cache)
                    .service(delete_media)
                    .service(copy_media)
                    .service(move_media),
            )
    })
    .bind(&bind_address)?;

    println!("Server is running at http://{}", bind_address);
    server.run().await
}
