extern crate cfg_if;
extern crate exif;

use std::sync::Arc;
use aws_types::region::Region;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log::LevelFilter;
use crate::adapter::S3;
use crate::api::server::run_server;
use crate::apikey::{ApiKeyStorage, RedisApiKeyStorage};
use crate::config::{ConfigLoader, StorageKind};
use crate::metadata::{MetadataStorage, RedisMetadataStorage};
use crate::scheduler::{RedisTaskStorage, TaskExecutor, TaskStorage};
use crate::scheduler::task_scheduler::run_scheduler;
use crate::storage::{FileStorage, FilesystemStorage, S3Storage};
use crate::transform::{NamedTransformationStorage, RedisNamedTransformationStorage};

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
mod utils;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let stdout = ConsoleAppender::builder().build();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    let config = ConfigLoader::load().unwrap();

    let redis_client = if let Some(redis) = config.adapter.redis.clone() {
        let redis_address = format!("redis://{}:{}", redis.host, redis.port);

        Some(redis::Client::open(redis_address.as_str()).expect("Error creating Redis client"))
    } else {
        None
    };

    let s3_client = if let Some(s3) = config.adapter.s3.clone() {
        let creds = aws_sdk_s3::config::Credentials::new(s3.access_key_id, s3.secret_access_key, None, None, "");
        let conf = aws_sdk_s3::Config::builder()
            .credentials_provider(creds)
            .region(Region::new(s3.region.clone()))
            .behavior_version_latest()
            .build();

        Some(aws_sdk_s3::Client::from_conf(conf))
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
        },
        StorageKind::S3 => panic!("S3 storage for apikeys is not supported yet"),
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
            },
            StorageKind::S3 => panic!("S3 storage for named transformations is not supported yet"),
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
        },
        StorageKind::S3 => panic!("S3 storage for metadata is not supported yet"),
    };

    let task_storage: Arc<dyn TaskStorage> = Arc::new(RedisTaskStorage::new(
        redis_client
            .as_ref()
            .unwrap()
            .get_connection()
            .expect("Error connecting to Redis"),
    ));

    let file_storage: Arc<dyn FileStorage> = match config.file_storage.storage_kind {
        StorageKind::Filesystem => Arc::new(FilesystemStorage::new(config.file_storage.filesystem.clone().unwrap().mount_dir)),
        StorageKind::S3 => Arc::new(S3Storage::new(S3::new(s3_client.clone().unwrap(), config.file_storage.s3.clone().unwrap().bucket_name))),
        StorageKind::Redis => panic!("Redis storage for files is not supported yet"),
    };

    let cache_storage: Arc<dyn FileStorage> = match config.cache_storage.storage_kind {
        StorageKind::Filesystem => Arc::new(FilesystemStorage::new(config.cache_storage.filesystem.clone().unwrap().mount_dir)),
        StorageKind::S3 => Arc::new(S3Storage::new(S3::new(s3_client.clone().unwrap(), config.cache_storage.s3.clone().unwrap().bucket_name))),
        StorageKind::Redis => panic!("Redis storage for cache is not supported yet"),
    };

    let clear_cache_task: Arc<dyn TaskExecutor> = Arc::new(handler::CacheHandler::new(
        cache_storage.clone(),
        metadata_storage.clone(),
    ));

    let task_executors = vec![
        (scheduler::TaskKind::ClearCache, clear_cache_task.clone())
    ]
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
