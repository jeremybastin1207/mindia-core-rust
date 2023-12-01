use actix_web::{App, HttpServer};
use dotenv::dotenv;
use redis::Client;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;

use crate::api::{get_apikeys, AppState};
use crate::apikey_storage::{ApiKeyStorage, RedisApiKeyStorage};

mod api;
mod apikey;
mod apikey_storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let client = match Client::open("redis://127.0.0.1:6379") {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error creating Redis client: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };
    let apikey_storage: Arc<Mutex<dyn ApiKeyStorage>> =
        Arc::new(Mutex::new(RedisApiKeyStorage::new(client).unwrap()));

    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("127.0.0.1:{}", port);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                apikey_storage: Arc::clone(&apikey_storage),
            })
            .service(get_apikeys)
    })
    .bind(&bind_address)?;

    println!("Server is running at http://{}", bind_address);

    server.run().await
}
