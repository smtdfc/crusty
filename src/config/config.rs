use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProxyConfig {
    pub port: i64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    pub api_key: String,

    pub current_model: Option<String>,

    pub proxy: Option<ProxyConfig>,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        if !config_path.exists() {
            return Err(
                "The configuration file does not exist. Please run crusty setup first!".into(),
            );
        }

        let config_str = fs::read_to_string(config_path)?;

        let config: AppConfig = serde_json::from_str(&config_str)?;

        Ok(config)
    }
    pub fn get_config_dir() -> PathBuf {
        let proj_dirs = ProjectDirs::from("io", "smtdfc", "crusty")
            .expect("The system configuration directory cannot be determined!");

        let config_dir = proj_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).ok();
        }

        config_dir.to_path_buf()
    }

    pub fn get_config_path() -> PathBuf {
        Self::get_config_dir().join("config.json")
    }
}
