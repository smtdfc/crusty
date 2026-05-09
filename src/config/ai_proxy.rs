use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AIProxyConfig {
    pub platform: String,
    pub is_local: bool,
    pub host: String,
    pub port: u64,
    pub api_key: Option<String>,
    pub current_model: Option<String>,
}
