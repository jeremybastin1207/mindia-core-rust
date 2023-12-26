#[derive(Debug)]
pub struct Config {
    pub master_key: String,
}

impl Config {
    pub fn new(master_key: String) -> Self {
        Config { master_key }
    }
}
