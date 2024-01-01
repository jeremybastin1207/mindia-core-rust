use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use super::{
    clear_cache, delete_apikey, delete_named_transformation, download_media, get_apikeys,
    get_named_transformations, get_transformation_templates, middleware_apikey::ApiKeyChecker,
    read_media, save_apikey, save_named_transformation, upload, AppState,
};
use crate::handler::{CacheHandler, MediaHandler};
use crate::transform::TransformationTemplateRegistry;
use crate::{apikey::ApiKeyStorage, transform::NamedTransformationStorage};
use crate::{config::Config, storage::FileStorage};
use crate::{metadata::MetadataStorage, scheduler::TaskScheduler};

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
                cache_handler: CacheHandler::new(
                    file_storage.clone(),
                    cache_storage.clone(),
                    metadata_storage.clone(),
                ),
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
                    .service(clear_cache),
            )
    })
    .bind(&bind_address)?;

    println!("Server is running at http://{}", bind_address);
    server.run().await
}
