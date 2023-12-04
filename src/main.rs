use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use redis::Client;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;

mod api;
mod apikey;
mod media;
mod named_transformation;
mod storage;
mod task;
mod transformation;
mod types;

use crate::api::{
    delete_apikey, delete_named_transformation, get_apikeys, get_named_transformations,
    save_apikey, save_named_transformation, upload, AppState,
};
use crate::apikey::{ApiKeyStorage, RedisApiKeyStorage};
use crate::named_transformation::{NamedTransformationStorage, RedisNamedTransformationStorage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let client = Client::open("redis://127.0.0.1:6379").expect("Error creating Redis client");

    let apikey_storage: Arc<Mutex<dyn ApiKeyStorage>> = Arc::new(Mutex::new(
        RedisApiKeyStorage::new(client.clone()).expect("Error creating RedisApiKeyStorage"),
    ));

    let named_transformation_storage: Arc<Mutex<dyn NamedTransformationStorage>> =
        Arc::new(Mutex::new(
            RedisNamedTransformationStorage::new(client.clone())
                .expect("Error creating RedisNamedTransformationStorage"),
        ));

    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("127.0.0.1:{}", port);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                apikey_storage: Arc::clone(&apikey_storage),
                named_transformation_storage: Arc::clone(&named_transformation_storage),
            })
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
