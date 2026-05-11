use tracing::trace;

use crate::{
    ai_proxy::ai_proxy::{AIProxy, get_proxy},
    config::{ai_proxy::AIProxyConfig, config::AppConfig},
    helpers::tui::print_error,
};

pub fn get_active_proxy(
    config: &AppConfig,
    action: &str,
) -> Option<(String, AIProxyConfig, Box<dyn AIProxy>)> {
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
            trace!("Proxy already installed, skipping...");
        }
    }

    Some((current_proxy, proxy_config, proxy))
}
