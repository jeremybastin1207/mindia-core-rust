#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKey {
    pub name: String,
    pub key: String,
}
