use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ProviderType {
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "anthropic")]
    Anthropic,
    #[serde(rename = "local-proxy")]
    LocalProxy,
    #[serde(rename = "custom")]
    Custom,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::OpenAI => write!(f, "openai"),
            ProviderType::Anthropic => write!(f, "anthropic"),
            ProviderType::LocalProxy => write!(f, "local-proxy"),
            ProviderType::Custom => write!(f, "custom"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProviderConfig {
    /// Provider type (openai, anthropic, local-proxy, custom)
    pub provider_type: String,

    /// API base URL (e.g., https://api.openai.com/v1 or http://localhost:8000/v1)
    pub base_url: String,

    /// API key for authentication
    pub api_key: String,

    /// Default model for this provider
    pub default_model: Option<String>,

    /// Additional metadata
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

impl ProviderConfig {
    pub fn normalize_base_url(base_url: &str) -> String {
        let mut normalized = base_url.trim().trim_end_matches('/').to_string();

        if normalized.starts_with("http://") || normalized.starts_with("https://") {
            let has_path = normalized
                .split_once("://")
                .map(|(_, rest)| rest.contains('/'))
                .unwrap_or(false);

            // OpenAI-compatible providers usually expose endpoints under /v1.
            if !has_path {
                normalized.push_str("/v1");
            }
        }

        normalized
    }

    pub fn new(
        provider_type: &str,
        base_url: &str,
        api_key: &str,
        default_model: Option<String>,
    ) -> Self {
        Self {
            provider_type: provider_type.to_string(),
            base_url: Self::normalize_base_url(base_url),
            api_key: api_key.to_string(),
            default_model,
            metadata: std::collections::HashMap::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.base_url.is_empty()
            && !self.api_key.is_empty()
            && (self.base_url.starts_with("http://") || self.base_url.starts_with("https://"))
    }

    pub fn get_provider_type(&self) -> ProviderType {
        match self.provider_type.as_str() {
            "openai" => ProviderType::OpenAI,
            "anthropic" => ProviderType::Anthropic,
            "local-proxy" => ProviderType::LocalProxy,
            _ => ProviderType::Custom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_config_validation() {
        let valid = ProviderConfig::new(
            "openai",
            "https://api.openai.com/v1",
            "sk-test",
            Some("gpt-4".to_string()),
        );
        assert!(valid.is_valid());

        let invalid = ProviderConfig::default();
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_provider_type_parsing() {
        let config = ProviderConfig::new("openai", "https://api.openai.com/v1", "key", None);
        assert_eq!(config.get_provider_type(), ProviderType::OpenAI);
    }
}
