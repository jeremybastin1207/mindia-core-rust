use opentelemetry::global;
use redis::Client;
use scheduler::task_scheduler::run_scheduler;
use scheduler::TaskExecutor;
use std::sync::Arc;

extern crate cfg_if;
extern crate exif;

mod adapter;
mod api;
mod apikey;
mod config;
mod extractor;
mod handler;
mod media;
mod metadata;
mod pipeline;
mod scheduler;
mod storage;
mod transform;
mod types;

use crate::api::run_server;
use crate::apikey::{ApiKeyStorage, RedisApiKeyStorage};
use crate::config::{ConfigLoader, StorageKind};
use crate::metadata::{MetadataStorage, RedisMetadataStorage};
use crate::scheduler::{RedisTaskStorage, TaskStorage};
use crate::storage::{FileStorage, FilesystemStorage};
use crate::transform::{NamedTransformationStorage, RedisNamedTransformationStorage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the global tracer
    global::set_text_map_propagator(opentelemetry::sdk::propagation::TraceContextPropagator::new());

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

    let file_storage: Arc<dyn FileStorage> = Arc::new(FilesystemStorage::new("./mnt/main"));

    let cache_storage: Arc<dyn FileStorage> = Arc::new(FilesystemStorage::new("./mnt/cache"));

    let clear_cache_task: Arc<dyn TaskExecutor> = Arc::new(handler::CacheHandler::new(
        cache_storage.clone(),
        metadata_storage.clone(),
    ));

    let task_executors = vec![(scheduler::TaskKind::ClearCache, clear_cache_task.clone())]
        .into_iter()
        .collect();
    let task_scheduler = run_scheduler(task_storage, task_executors);

    run_server(
        config,
        file_storage,
        cache_storage,
        metadata_storage,
        named_transformation_storage,
        apikey_storage,
        task_scheduler,
    )
    .await
}
