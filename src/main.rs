use redis::Client;
use scheduler::task_scheduler::run_scheduler;
use scheduler::TaskExecutor;
use std::sync::Arc;
use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log::LevelFilter;
use opentelemetry::global;
use opentelemetry::sdk::trace::TracerProvider;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry_otlp::{ExportConfig, SpanExporter, TonicConfig};
use tonic::transport::Channel;

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
mod utils;

use crate::{
    api::run_server,
    apikey::{ApiKeyStorage, RedisApiKeyStorage},
    config::{ConfigLoader, StorageKind},
    metadata::{MetadataStorage, RedisMetadataStorage},
    scheduler::{RedisTaskStorage, TaskStorage},
    storage::{FileStorage, FilesystemStorage},
    transform::{NamedTransformationStorage, RedisNamedTransformationStorage},
};

fn init_tracer() {
    let otlp_collector_url = "https://otlp-gateway-prod-eu-west-0.grafana.net/otlp";

    let export_config = ExportConfig {
        endpoint: otlp_collector_url.to_string(),
        ..Default::default()
    };

    match  SpanExporter::new_tonic(export_config, TonicConfig::default())  {
        Ok(exporter) => {
            let provider = TracerProvider::builder()
                .with_simple_exporter(exporter)
                .build();
            global::set_tracer_provider(provider);
            global::set_text_map_propagator(TraceContextPropagator::new());
        },
        Err(why) => panic!("{:?}", why)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    //init_tracer();

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
