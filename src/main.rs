use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
// use env_logger;
// use env_logger::Env;
use redis::Client;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;

extern crate cfg_if;
extern crate exif;

mod adapter;
mod api;
mod apikey;
mod extractor;
mod media;
mod metadata;
mod named_transformation;
mod storage;
mod task;
mod transform;
mod types;

use crate::adapter::s3::S3;
use crate::api::{
    delete_apikey, delete_named_transformation, get_apikeys, get_named_transformations,
    save_apikey, save_named_transformation, upload, AppState,
};
use crate::apikey::{ApiKeyStorage, RedisApiKeyStorage};
use crate::metadata::{MetadataStorage, RedisMetadataStorage};
use crate::named_transformation::{NamedTransformationStorage, RedisNamedTransformationStorage};
use crate::storage::{FileStorage, FilesystemStorage, S3Storage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    //std::env::set_var("RUST_LOG", "debug");
    //env_logger::init();

    dotenv().expect("Failed to read .env file");

    let redis_client = Client::open("redis://127.0.0.1:6379").expect("Error creating Redis client");

    let s3_client = S3::new().await.unwrap();

    let apikey_storage: Arc<Mutex<dyn ApiKeyStorage>> = Arc::new(Mutex::new(
        RedisApiKeyStorage::new(redis_client.clone()).expect("Error creating RedisApiKeyStorage"),
    ));

    let named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>> =
        Arc::new(Mutex::new(
            RedisNamedTransformationStorage::new(redis_client.clone())
                .expect("Error creating RedisNamedTransformationStorage"),
        ));

    let metadata_storage: Arc<Mutex<dyn MetadataStorage>> = Arc::new(Mutex::new(
        RedisMetadataStorage::new(redis_client.clone())
            .expect("Error creating RedisMetadataStorage"),
    ));

    let _s3_storage: Arc<Mutex<dyn FileStorage>> = Arc::new(Mutex::new(S3Storage::new(s3_client)));
    let filesystem_storage: Arc<Mutex<dyn FileStorage>> =
        Arc::new(Mutex::new(FilesystemStorage::new()));

    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("127.0.0.1:{}", port);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                apikey_storage: Arc::clone(&apikey_storage),
                named_transformation_storage: Arc::clone(&named_transformation_storage),
                upload_media: Arc::new(task::UploadMedia::new(
                    Arc::clone(&filesystem_storage),
                    Arc::clone(&metadata_storage),
                    Arc::clone(&named_transformation_storage),
                )),
            }))
            .service(
                web::scope("/api/v0")
                    .service(get_apikeys)
                    .service(save_apikey)
                    .service(delete_apikey)
                    .service(get_named_transformations)
                    .service(save_named_transformation)
                    .service(delete_named_transformation)
                    .service(upload),
            )
    })
    .bind(&bind_address)?;

    println!("Server is running at http://{}", bind_address);

    server.run().await
}
