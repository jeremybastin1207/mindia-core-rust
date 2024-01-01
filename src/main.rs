use actix_web::{web, App, HttpServer};
use redis::Client;
use std::sync::Arc;

extern crate cfg_if;
extern crate exif;

mod adapter;
mod api;
mod apikey;
mod config;
mod extractor;
mod media;
mod metadata;
mod named_transformation;
mod pipeline;
mod scheduler;
mod storage;
mod task;
mod transform;
mod types;

use crate::api::{
    clear_cache, delete_apikey, delete_named_transformation, download_media, get_apikeys,
    get_named_transformations, get_transformation_templates, read_media, save_apikey,
    save_named_transformation, upload, AppState,
};
use crate::apikey::{ApiKeyStorage, RedisApiKeyStorage};
use crate::config::{ConfigLoader, StorageKind};
use crate::metadata::{MetadataStorage, RedisMetadataStorage};
use crate::named_transformation::{NamedTransformationStorage, RedisNamedTransformationStorage};
use crate::scheduler::{RedisTaskStorage, TaskScheduler, TaskStorage};
use crate::storage::{FileStorage, FilesystemStorage};
use crate::transform::TransformationTemplateRegistry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ConfigLoader::load().unwrap();

    let redis_client = if let Some(redis) = config.adapter.redis.clone() {
        let redis_address = format!("redis://{}:{}", redis.host, redis.port);

        Some(Client::open(redis_address.as_str()).expect("Error creating Redis client"))
    } else {
        None
    };

    let apikey_storage: Arc<dyn ApiKeyStorage> = match config.apikey.storage_kind {
        StorageKind::Filesystem => panic!("Filesystem storage for apikeys is not supported yet"),
        StorageKind::Redis => {
            let redis_conn = redis_client
                .as_ref()
                .unwrap()
                .get_connection()
                .expect("Error connecting to Redis");
            Arc::new(
                RedisApiKeyStorage::new(redis_conn).expect("Error creating RedisApiKeyStorage"),
            )
        }
    };

    let named_transformation_storage: Arc<dyn NamedTransformationStorage> =
        match config.named_transformation.storage_kind {
            StorageKind::Filesystem => {
                panic!("Filesystem storage for named transformations is not supported yet")
            }
            StorageKind::Redis => {
                let redis_conn = redis_client
                    .as_ref()
                    .unwrap()
                    .get_connection()
                    .expect("Error connecting to Redis");
                Arc::new(
                    RedisNamedTransformationStorage::new(redis_conn)
                        .expect("Error creating RedisNamedTransformationStorage"),
                )
            }
        };

    let metadata_storage: Arc<dyn MetadataStorage> = match config.metadata.storage_kind {
        StorageKind::Filesystem => {
            panic!("Filesystem storage for metadata is not supported yet")
        }
        StorageKind::Redis => {
            let redis_conn = redis_client
                .as_ref()
                .unwrap()
                .get_connection()
                .expect("Error connecting to Redis");
            Arc::new(RedisMetadataStorage::new(redis_conn))
        }
    };

    let task_storage: Arc<dyn TaskStorage> = Arc::new(RedisTaskStorage::new(
        redis_client
            .as_ref()
            .unwrap()
            .get_connection()
            .expect("Error connecting to Redis"),
    ));

    let filesystem_file_storage: Arc<dyn FileStorage> =
        Arc::new(FilesystemStorage::new("./mnt/main"));

    let filesystem_cache_storage: Arc<dyn FileStorage> =
        Arc::new(FilesystemStorage::new("./mnt/cache"));

    let clear_cache_task = Arc::new(task::ClearCache::new(
        filesystem_file_storage.clone(),
        filesystem_cache_storage.clone(),
        metadata_storage.clone(),
    ));

    let mut task_scheduler = TaskScheduler::new(Arc::clone(&task_storage));
    task_scheduler.register_task_executor(scheduler::TaskKind::ClearCache, clear_cache_task);

    let task_scheduler = Arc::new(task_scheduler);

    let task_scheduler_clone = Arc::clone(&task_scheduler);
    let task_scheduler_clone_for_shutdown = Arc::clone(&task_scheduler);

    actix_web::rt::spawn(async move {
        task_scheduler_clone.run().await;
    });

    ctrlc::set_handler(move || {
        task_scheduler_clone_for_shutdown.stop();
        actix_web::rt::System::current().stop();
    })
    .expect("Error setting Ctrl-C handler");

    let bind_address = format!("127.0.0.1:{}", config.server.port.clone());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_cors::Cors::default())
            .wrap(api::middleware_apikey::ApiKeyChecker::new(
                apikey_storage.clone(),
                config.master_key.clone(),
            ))
            .app_data(web::Data::new(AppState {
                apikey_storage: apikey_storage.clone(),
                named_transformation_storage: named_transformation_storage.clone(),
                transformation_template_registry: Arc::new(TransformationTemplateRegistry::new()),
                upload_media: Arc::new(task::UploadMedia::new(
                    filesystem_file_storage.clone(),
                    filesystem_cache_storage.clone(),
                    metadata_storage.clone(),
                )),
                read_media: Arc::new(task::ReadMedia::new(metadata_storage.clone())),
                download_media: Arc::new(task::DownloadMedia::new(
                    filesystem_file_storage.clone(),
                    filesystem_cache_storage.clone(),
                    metadata_storage.clone(),
                )),
                delete_media: Arc::new(task::DeleteMedia::new(
                    filesystem_file_storage.clone(),
                    filesystem_cache_storage.clone(),
                    metadata_storage.clone(),
                )),
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
