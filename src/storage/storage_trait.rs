pub trait Storage {
    fn upload(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}
