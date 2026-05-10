use clap::Subcommand;

use crate::{
    cli::utils::get_active_proxy,
    config::config::AppConfig,
    helpers::tui::{print_error, print_info, show_loading},
};

#[derive(Subcommand)]
pub enum ProxyCommands {
    /// Start the active AI proxy service
    Start {},
    /// Stop the currently running AI proxy service
    Stop {},
    /// Open the AI proxy dashboard in the browser
    Dashboard {},
}

pub fn handle_proxy_start() {
    show_loading("Preparing ...");
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            print_error(&format!("{}", e));
            return;
        }
    };
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy(&config, "start") else {
        return;
    };

    if !proxy.is_running() {
        match proxy.start() {
            Ok(()) => {
                print_info(&format!(
                    "Proxy {} (platform: {}) is running on port {}",
                    current_proxy, proxy_config.platform, proxy_config.port
                ));
            }

            Err(e) => {
                print_error(&format!(
                    "Cannot start proxy {} (platform: {}) on port {}. Please check log for details.",
                    current_proxy, proxy_config.platform, proxy_config.port
                ));
            }
        }
    } else {
        print_info(&format!(
            "Proxy {} (platform: {}) is running on port {}",
            current_proxy, proxy_config.platform, proxy_config.port
        ));
    }
}

pub fn handle_proxy_stop() {
    show_loading("Preparing ...");
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            print_error(&format!("{}", e));
            return;
        }
    };
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy(&config, "stop") else {
        return;
    };

    if proxy.is_running() {
        match proxy.stop() {
            Ok(()) => {
                print_info(&format!(
                    "Proxy {} (platform: {}) is stopped",
                    current_proxy, proxy_config.platform
                ));
            }

            Err(e) => {
                print_error(&format!(
                    "Cannot stop proxy {} (platform: {}) on port {}. Please check log for details.",
                    current_proxy, proxy_config.platform, proxy_config.port
                ));
            }
        }
    } else {
        print_info(&format!(
            "Proxy {} (platform: {}) not run",
            current_proxy, proxy_config.platform
        ));
    }
}
