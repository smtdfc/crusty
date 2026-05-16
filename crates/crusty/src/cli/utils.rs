use tracing::{error, info};

use crate::{
    agent::memory::store::{SharedMemoryStore, get_store},
    ai_proxy::ai_proxy::{AIProxy, get_proxy},
    config::{
        ai_proxy::AIProxyConfig,
        config::{AppConfig, GLOBAL_CONFIG},
        provider::ProviderConfig,
    },
    helpers::tui::print_error,
};

use tokio::sync::OnceCell;

pub type ActiveProxy = (String, AIProxyConfig, Box<dyn AIProxy>);
pub type ActiveProvider = (String, ProviderConfig);

pub fn get_active_proxy(config: &AppConfig, action: &str) -> Option<ActiveProxy> {
    let Some(current_proxy) = config.current_proxy.clone() else {
        print_error("No proxy select. Please setup first.");
        return None;
    };

    let Some(proxy_config) = config.ai_proxies.get(&current_proxy).cloned() else {
        print_error("No proxy found. Please setup first.");
        return None;
    };

    if !proxy_config.is_local {
        print_error(&format!(
            "Proxy {} (platform: {}) is remote proxy from another address cannot {} locally",
            current_proxy, proxy_config.platform, action
        ));
    }

    let Some(proxy) = get_proxy(&proxy_config.platform, &proxy_config) else {
        print_error("Failed to init proxy. Please check logs for details.");
        return None;
    };

    match proxy.is_install() {
        Err(_) => {
            print_error(&format!(
                "Platform {} (for {}) not install. Please run crusty setup first.",
                proxy_config.platform, current_proxy
            ));
            return None;
        }

        Ok(_) => {
            info!("Proxy already installed, skipping...");
        }
    }

    Some((current_proxy, proxy_config, proxy))
}

pub fn get_active_proxy_and_check(action: &str, check_running: bool) -> Option<ActiveProxy> {
    let config = GLOBAL_CONFIG.read().unwrap();
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy(&config, action) else {
        return None;
    };

    if check_running {
        match proxy.is_running() {
            Ok(false) => {
                print_error(&format!(
                    "Proxy {} (platform: {}) is offline. Please run proxy before.",
                    current_proxy, proxy_config.platform
                ));
                return None;
            }
            Err(e) => {
                error!(error = ?e, "Failed to check proxy status");
                print_error(&format!(
                    "Cannot check status of proxy {} (platform: {}) on port {}. Please check log for details.",
                    current_proxy, proxy_config.platform, proxy_config.port
                ));
                return None;
            }
            Ok(true) => {}
        }
    }

    Some((current_proxy, proxy_config, proxy))
}

pub fn get_agent_params(proxy_config: &AIProxyConfig) -> Option<(String, String)> {
    let Some(model_name) = proxy_config.current_model.clone() else {
        print_error("No model select. Please select a model to start chat.");
        return None;
    };

    let api_key = match proxy_config.api_key.as_deref() {
        None => String::from(""),
        Some(v) => v.to_string(),
    };

    Some((model_name, api_key))
}

/// Get the active provider and its configuration
pub fn get_active_provider(config: &AppConfig) -> Option<ActiveProvider> {
    let Some(current_provider) = config.current_provider.clone() else {
        print_error("No provider configured. Please run 'crusty provider add' to add a provider.");
        return None;
    };

    let Some(provider_config) = config.find_provider_by_id(&current_provider) else {
        print_error("No provider found. Please setup first.");
        return None;
    };

    if !provider_config.is_valid() {
        print_error("Provider configuration is invalid (missing base_url or api_key).");
        return None;
    }

    Some((current_provider, provider_config))
}

/// Get the active provider and check availability
pub fn get_active_provider_and_check() -> Option<ActiveProvider> {
    let config = GLOBAL_CONFIG.read().unwrap();
    get_active_provider(&config)
}

static STORE_CACHE: OnceCell<Option<SharedMemoryStore>> = OnceCell::const_new();

pub async fn get_initialized_store() -> Option<SharedMemoryStore> {
    let store_opt = STORE_CACHE.get_or_init(|| async {
        let store_config = {
            let config = GLOBAL_CONFIG.read().unwrap();
            config.store.clone()
        };

        let Some(store_config) = store_config else {
            print_error("Store not configured. Please setup your store.");
            return None;
        };

        match get_store(&store_config).await {
            Ok(s) => Some(s),
            Err(e) => {
                error!(error = ?e, "Failed to create store");
                print_error(&format!("Cannot init chat session now. Cause: {}", e));
                None
            }
        }
    }).await;

    store_opt.clone()
}
