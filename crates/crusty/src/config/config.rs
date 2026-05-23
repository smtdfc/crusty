use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};
use tracing::error;

use crate::config::ai_proxy::AIProxyConfig;
use crate::config::plugin::PluginConfig;
use crate::config::provider::ProviderConfig;
use crate::config::store::StoreConfig;
use crate::exceptions::crusty::CrustyError;
use crate::helpers::tui::print_error;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum RunMode {
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "provider")]
    Provider,
}

impl std::fmt::Display for RunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunMode::Proxy => write!(f, "proxy"),
            RunMode::Provider => write!(f, "provider"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    /// Current running mode (proxy or provider)
    #[serde(default)]
    pub mode: Option<RunMode>,

    /// Legacy field for backward compatibility with proxy-based config
    pub current_proxy: Option<String>,
    pub ai_proxies: HashMap<String, AIProxyConfig>,

    /// New field for OpenAI-compatible providers
    #[serde(default)]
    pub current_provider: Option<String>,
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,

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

    pub fn find_proxy_by_id(&self, name: &str) -> Option<AIProxyConfig> {
        self.ai_proxies.get(name).cloned()
    }

    /// Find the currently active proxy configuration
    pub fn get_current_proxy(&self) -> Option<AIProxyConfig> {
        self.current_proxy
            .as_ref()
            .and_then(|name| self.find_proxy_by_id(name))
    }

    /// Set the currently active proxy
    pub fn set_current_proxy(&mut self, name: Option<String>) -> Result<(), CrustyError> {
        if let Some(ref proxy_name) = name {
            if !self.ai_proxies.contains_key(proxy_name) {
                return Err(CrustyError::ConfigError(format!(
                    "Proxy '{}' not found",
                    proxy_name
                )));
            }
        }

        self.current_proxy = name;
        Ok(())
    }

    /// Add a new proxy configuration
    pub fn add_proxy(&mut self, name: String, config: AIProxyConfig) -> Result<(), CrustyError> {
        self.ai_proxies.insert(name, config);

        if self.current_proxy.is_none() && !self.ai_proxies.is_empty() {
            self.current_proxy = self.ai_proxies.keys().next().cloned();
        }

        Ok(())
    }

    /// Remove a proxy configuration
    pub fn remove_proxy(&mut self, name: &str) -> Result<(), CrustyError> {
        self.ai_proxies.remove(name);

        if let Some(ref current) = self.current_proxy {
            if current == name {
                self.current_proxy = self.ai_proxies.keys().next().cloned();
            }
        }

        Ok(())
    }

    /// List all proxy names
    pub fn list_proxies(&self) -> Vec<String> {
        self.ai_proxies.keys().cloned().collect()
    }

    /// Find a provider configuration by name
    pub fn find_provider_by_id(&self, name: &str) -> Option<ProviderConfig> {
        self.providers.get(name).cloned()
    }

    /// Get the currently active provider configuration
    pub fn get_current_provider(&self) -> Option<ProviderConfig> {
        self.current_provider
            .as_ref()
            .and_then(|name| self.find_provider_by_id(name))
    }

    /// Set the currently active provider
    pub fn set_current_provider(&mut self, name: Option<String>) -> Result<(), CrustyError> {
        if let Some(ref provider_name) = name {
            if !self.providers.contains_key(provider_name) {
                return Err(CrustyError::ConfigError(format!(
                    "Provider '{}' not found",
                    provider_name
                )));
            }
        }
        self.current_provider = name;
        Ok(())
    }

    /// Add a new provider configuration
    pub fn add_provider(
        &mut self,
        name: String,
        config: ProviderConfig,
    ) -> Result<(), CrustyError> {
        if !config.is_valid() {
            return Err(CrustyError::ConfigError(
                "Provider configuration is invalid (missing base_url or api_key)".into(),
            ));
        }
        self.providers.insert(name, config);
        Ok(())
    }

    /// Remove a provider configuration
    pub fn remove_provider(&mut self, name: &str) -> Result<(), CrustyError> {
        self.providers.remove(name);

        // If the removed provider was active, clear the current provider
        if let Some(ref current) = self.current_provider {
            if current == name {
                self.current_provider = None;
            }
        }
        Ok(())
    }

    /// List all provider names
    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    /// Get the current running mode (requires mode to be explicitly set)
    pub fn get_mode(&self) -> Result<RunMode, CrustyError> {
        match self.mode {
            Some(mode) => Ok(mode),
            None => Err(CrustyError::ConfigError(
                "No mode selected. Please run 'crusty mode switch' to choose between 'proxy' and 'provider' mode.".into(),
            )),
        }
    }

    /// Set the running mode
    pub fn set_mode(&mut self, mode: RunMode) -> Result<(), CrustyError> {
        // Validate that the chosen mode has necessary configuration
        match mode {
            RunMode::Proxy => {
                if self.current_proxy.is_none() || self.ai_proxies.is_empty() {
                    return Err(CrustyError::ConfigError(
                        "No proxy configured. Please run 'crusty setup' first.".into(),
                    ));
                }
            }
            RunMode::Provider => {
                if self.current_provider.is_none() || self.providers.is_empty() {
                    return Err(CrustyError::ConfigError(
                        "No provider configured. Please run 'crusty provider add' first.".into(),
                    ));
                }
            }
        }

        self.mode = Some(mode);
        Ok(())
    }

    /// Check if currently in provider mode (returns error if mode not set)
    pub fn is_provider_mode(&self) -> Result<bool, CrustyError> {
        Ok(self.get_mode()? == RunMode::Provider)
    }

    /// Check if currently in proxy mode (returns error if mode not set)
    pub fn is_proxy_mode(&self) -> Result<bool, CrustyError> {
        Ok(self.get_mode()? == RunMode::Proxy)
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
