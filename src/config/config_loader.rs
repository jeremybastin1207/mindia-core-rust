use dotenv::dotenv;
use std::error::Error;
use std::fs;
use std::process::exit;
use toml;

use super::Config;

pub struct ConfigLoader {}

impl ConfigLoader {
    pub fn load() -> Result<Config, Box<dyn Error>> {
        match dotenv() {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to load .env file");
                exit(1);
            }
        }

        let master_key = std::env::var("MASTER_KEY").map_err(|_| "MASTER_KEY must be set")?;

        let config_path = "config.toml";
        let config_str = fs::read_to_string(config_path)
            .map_err(|_| format!("Failed to read config file: {}", config_path))?;

        let mut config: Config = toml::from_str(&config_str)
            .map_err(|err| format!("Failed to parse config file: {}, {}", config_path, err))?;

        if config.server.port == 0 {
            config.server.port = 8080;
        }

        config.master_key = master_key;

        Ok(config)
    }
}
