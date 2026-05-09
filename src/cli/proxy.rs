use clap::Subcommand;


use crate::{
    cli::utils::get_active_proxy,
    helpers::tui::{print_error, print_info, show_loading},
};

#[derive(Subcommand)]
pub enum ProxyCommands {
    Start {},
    Stop {},
    Dashboard {},
}

pub fn handle_proxy_start() {
    show_loading("Preparing ...");
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy("start") else {
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
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy("stop") else {
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
