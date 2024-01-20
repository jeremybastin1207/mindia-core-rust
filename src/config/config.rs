use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisAdapterConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct S3AdapterConfig {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub endpoint: String,
    pub region: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilesystemStorageConfig {
    pub mount_dir: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct S3StorageConfig {
    pub bucket_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdapterConfig {
    pub redis: Option<RedisAdapterConfig>,
    pub s3: Option<S3AdapterConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub storage_kind: StorageKind,
    pub filesystem: Option<FilesystemStorageConfig>,
    pub s3: Option<S3StorageConfig>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StorageKind {
    Filesystem,
    Redis,
    S3,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiKeyConfig {
    pub storage_kind: StorageKind,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NamedTransformationConfig {
    pub storage_kind: StorageKind,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetadataConfig {
    pub storage_kind: StorageKind,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub master_key: String,
    pub server: ServerConfig,
    pub adapter: AdapterConfig,
    pub file_storage: StorageConfig,
    pub cache_storage: StorageConfig,
    pub apikey: ApiKeyConfig,
    pub named_transformation: NamedTransformationConfig,
    pub metadata: MetadataConfig,
}
