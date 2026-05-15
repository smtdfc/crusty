use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};
use tracing::error;

use crate::config::ai_proxy::AIProxyConfig;
use crate::config::plugin::PluginConfig;
use crate::config::store::StoreConfig;
use crate::exceptions::crusty::CrustyError;
use crate::helpers::tui::print_error;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    pub current_proxy: Option<String>,
    pub ai_proxies: HashMap<String, AIProxyConfig>,
    pub plugins: Vec<PluginConfig>,
    pub store: Option<StoreConfig>,
}

impl AppConfig {
    pub fn load() -> Result<Self, CrustyError> {
        let config_path = Self::get_config_path();
        if !config_path.exists() {
            return Err(CrustyError::ConfigError(
                "The configuration file does not exist. Please run crusty setup first!".into(),
            ));
        }

        let config_str = fs::read_to_string(config_path).map_err(|e| {
            return CrustyError::ConfigError(format!("Failed to load config. Cause: {}", e));
        })?;

        let config: AppConfig = serde_json::from_str(&config_str).map_err(|e| {
            return CrustyError::ConfigError(format!("Failed to load config. Cause: {}", e));
        })?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), CrustyError> {
        let config_path = Self::get_config_path();
        let config_str = serde_json::to_string_pretty(self).map_err(|e| {
            return CrustyError::ConfigError(format!("Failed to load config. Cause: {}", e));
        })?;

        let config_dir = Self::get_config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).map_err(|e| {
                return CrustyError::ConfigError(format!("Failed to save config. Cause: {}", e));
            })?;
        }

        fs::write(config_path, config_str).map_err(|e| {
            return CrustyError::ConfigError(format!("Failed to load config. Cause: {}", e));
        })?;

        Ok(())
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

    pub fn get_data_dir() -> PathBuf {
        let proj_dirs = ProjectDirs::from("io", "smtdfc", "crusty")
            .expect("The system configuration directory cannot be determined!");

        let data_dir = proj_dirs.data_dir();
        if !data_dir.exists() {
            fs::create_dir_all(data_dir).ok();
        }

        data_dir.to_path_buf()
    }

    pub fn get_config_path() -> PathBuf {
        Self::get_config_dir().join("config.json")
    }

    pub fn find_proxy_by_id(&self, name: &String) -> Option<AIProxyConfig> {
        self.ai_proxies.get(name).cloned()
    }
}

pub static GLOBAL_CONFIG: LazyLock<RwLock<AppConfig>> = LazyLock::new(|| {
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(error = ?e, "Failed to load config");
            print_error(&format!("Failed to load config"));
            panic!("");
        }
    };

    RwLock::new(config)
});
