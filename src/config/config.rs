use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct FilesystemConfig {
    pub mount_dir: String,
}

#[derive(Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Clone, Deserialize)]
pub struct AdapterConfig {
    pub redis: Option<RedisConfig>,
}

#[derive(Clone, Deserialize)]
pub struct StorageConfig {
    pub filesystem: Option<FilesystemConfig>,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StorageKind {
    Filesystem,
    Redis,
}

#[derive(Clone, Deserialize)]
pub struct ApiKeyConfig {
    pub storage_kind: StorageKind,
}

#[derive(Clone, Deserialize)]
pub struct NamedTransformationConfig {
    pub storage_kind: StorageKind,
}

#[derive(Clone, Deserialize)]
pub struct MetadataConfig {
    pub storage_kind: StorageKind,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub master_key: String,
    pub server: ServerConfig,
    pub adapter: AdapterConfig,
    pub storage: StorageConfig,
    pub apikey: ApiKeyConfig,
    pub named_transformation: NamedTransformationConfig,
    pub metadata: MetadataConfig,
}
