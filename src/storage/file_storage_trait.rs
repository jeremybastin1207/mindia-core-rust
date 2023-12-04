pub trait FileStorage {
    fn upload(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}
