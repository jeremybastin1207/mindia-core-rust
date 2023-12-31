use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
// use env_logger;
// use env_logger::Env;
use redis::Client;
use std::env;
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

use crate::adapter::s3::S3;
use crate::api::{
    clear_cache, delete_apikey, delete_named_transformation, download_media, get_apikeys,
    get_named_transformations, get_transformation_templates, read_media, save_apikey,
    save_named_transformation, upload, AppState,
};
use crate::apikey::{ApiKeyStorage, RedisApiKeyStorage};
use crate::config::Config;
use crate::metadata::{MetadataStorage, RedisMetadataStorage};
use crate::named_transformation::{NamedTransformationStorage, RedisNamedTransformationStorage};
use crate::scheduler::{RedisTaskStorage, TaskScheduler, TaskStorage};
use crate::storage::{FileStorage, FilesystemStorage, S3Storage};
use crate::transform::TransformationTemplateRegistry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    //std::env::set_var("RUST_LOG", "debug");
    //env_logger::init();

    dotenv().expect("Failed to read .env file");

    let master_key = env::var("MASTER_KEY").expect("MASTER_KEY must be set");

    let redis_client = Client::open("redis://127.0.0.1:6379").expect("Error creating Redis client");

    let s3_client = S3::new().await.unwrap();

    let apikey_storage: Arc<dyn ApiKeyStorage> = Arc::new(
        RedisApiKeyStorage::new(redis_client.get_connection().unwrap())
            .expect("Error creating RedisApiKeyStorage"),
    );

    let named_transformation_storage: Arc<dyn NamedTransformationStorage> = Arc::new(
        RedisNamedTransformationStorage::new(redis_client.get_connection().unwrap())
            .expect("Error creating RedisNamedTransformationStorage"),
    );

    let metadata_storage: Arc<dyn MetadataStorage> = Arc::new(RedisMetadataStorage::new(
        redis_client.get_connection().unwrap(),
    ));

    let task_storage: Arc<dyn TaskStorage> = Arc::new(RedisTaskStorage::new(
        redis_client.get_connection().unwrap(),
    ));

    let _s3_storage: Arc<dyn FileStorage> = Arc::new(S3Storage::new(s3_client));
    let filesystem_file_storage: Arc<dyn FileStorage> =
        Arc::new(FilesystemStorage::new("./mnt/main"));

    let filesystem_cache_storage: Arc<dyn FileStorage> =
        Arc::new(FilesystemStorage::new("./mnt/cache"));

    let clear_cache_task = Arc::new(task::ClearCache::new(
        Arc::clone(&filesystem_file_storage),
        Arc::clone(&filesystem_cache_storage),
        Arc::clone(&metadata_storage),
    ));

    let mut task_scheduler = TaskScheduler::new(Arc::clone(&task_storage));
    task_scheduler.register_task_executor(scheduler::TaskKind::ClearCache, clear_cache_task);

    let task_scheduler = Arc::new(task_scheduler);

    let task_scheduler_thread = {
        let task_scheduler = task_scheduler.clone();
        std::thread::spawn(move || {
            task_scheduler.run().unwrap();
        })
    };

    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("127.0.0.1:{}", port);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_cors::Cors::default())
            .wrap(api::middleware_apikey::ApiKeyChecker::new(
                Arc::clone(&apikey_storage),
                master_key.clone(),
            ))
            .app_data(web::Data::new(AppState {
                apikey_storage: Arc::clone(&apikey_storage),
                named_transformation_storage: Arc::clone(&named_transformation_storage),
                transformation_template_registry: Arc::new(TransformationTemplateRegistry::new()),
                upload_media: Arc::new(task::UploadMedia::new(
                    Arc::clone(&filesystem_file_storage),
                    Arc::clone(&filesystem_cache_storage),
                    Arc::clone(&metadata_storage),
                )),
                read_media: Arc::new(task::ReadMedia::new(Arc::clone(&metadata_storage))),
                download_media: Arc::new(task::DownloadMedia::new(
                    Arc::clone(&filesystem_file_storage),
                    Arc::clone(&filesystem_cache_storage),
                    Arc::clone(&metadata_storage),
                )),
                delete_media: Arc::new(task::DeleteMedia::new(
                    Arc::clone(&filesystem_file_storage),
                    Arc::clone(&filesystem_cache_storage),
                    Arc::clone(&metadata_storage),
                )),
                task_scheduler: task_scheduler.clone(),
                config: Arc::new(Config::new(master_key.clone())),
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
