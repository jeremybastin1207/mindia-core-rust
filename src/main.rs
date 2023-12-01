use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use redis::Client;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;

mod api;
mod apikey;
mod apikey_storage;
mod app_state;

use crate::api::{delete_apikey, get_apikeys, save_apikey};
use crate::apikey_storage::{ApiKeyStorage, RedisApiKeyStorage};
use crate::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let client = Client::open("redis://127.0.0.1:6379").expect("Error creating Redis client");

    let apikey_storage: Arc<Mutex<dyn ApiKeyStorage>> = Arc::new(Mutex::new(
        RedisApiKeyStorage::new(client).expect("Error creating RedisApiKeyStorage"),
    ));

    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("127.0.0.1:{}", port);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                apikey_storage: Arc::clone(&apikey_storage),
            })
            .service(
                web::scope("/api/v0")
                    .service(get_apikeys)
                    .service(save_apikey)
                    .service(delete_apikey),
            )
    })
    .bind(&bind_address)?;

    println!("Server is running at http://{}", bind_address);

    server.run().await
}
