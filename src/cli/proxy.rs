use clap::Subcommand;
use console::style;

use crate::{
    ai_proxy::ai_proxy::{AIProxy, get_proxy},
    config::config::AppConfig,
    helpers::tui::show_loading,
};

#[derive(Subcommand)]
pub enum ProxyCommands {
    Start {},
    Stop {},
    Dashboard {},
}

pub fn handle_proxy_start() {
    show_loading("Preparing ...");
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);

            return;
        }
    };

    let Some(current_proxy) = &config.current_proxy else {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            "No proxy select. Please setup first."
        );
        return;
    };

    let Some(proxy_config) = config.find_proxy_by_id(current_proxy) else {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            "No proxy found. Please setup first."
        );
        return;
    };

    if !proxy_config.is_local {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            format!(
                "Proxy {} (platform: {}) is remote proxy from another address cannot start locally",
                current_proxy, proxy_config.platform
            )
        );
    }

    let Some(proxy) = get_proxy(&proxy_config.platform, &proxy_config) else {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            "Failed to init proxy. Please check logs for details."
        );
        return;
    };

    if !proxy.is_install() {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            format!(
                "Platform {} (for {}) not install. Please run crusty setup first.",
                proxy_config.platform, current_proxy
            )
        );
        return;
    };

    if !proxy.is_running() {
        match proxy.start() {
            Ok(()) => {
                println!(
                    "{} {}",
                    style("Info:").blue().bold(),
                    format!(
                        "Proxy {} (platform: {}) is running on port {}",
                        current_proxy, proxy_config.platform, proxy_config.port
                    )
                )
            }

            Err(e) => {
                eprintln!(
                    "{} {}",
                    style("Error:").red().bold(),
                    format!(
                        "Cannot start proxy {} (platform: {}) on port {}. Please check log for details.",
                        current_proxy, proxy_config.platform, proxy_config.port
                    )
                );
            }
        }
    } else {
        println!(
            "{} {}",
            style("Info:").blue().bold(),
            format!(
                "Proxy {} (platform: {}) is running on port {}",
                current_proxy, proxy_config.platform, proxy_config.port
            )
        )
    }
}

pub fn handle_proxy_stop() {
    show_loading("Preparing ...");
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);

            return;
        }
    };

    let Some(current_proxy) = &config.current_proxy else {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            "No proxy select. Please setup first."
        );
        return;
    };

    let Some(proxy_config) = config.find_proxy_by_id(current_proxy) else {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            "No proxy found. Please setup first."
        );
        return;
    };

    if !proxy_config.is_local {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            format!(
                "Proxy {} (platform: {}) is remote proxy from another address cannot stop locally",
                current_proxy, proxy_config.platform
            )
        );
    }

    let Some(proxy) = get_proxy(&proxy_config.platform, &proxy_config) else {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            "Failed to init proxy. Please check logs for details."
        );
        return;
    };

    if proxy.is_running() {
        match proxy.stop() {
            Ok(()) => {
                println!(
                    "{} {}",
                    style("Info:").blue().bold(),
                    format!(
                        "Proxy {} (platform: {}) is stopped",
                        current_proxy, proxy_config.platform
                    )
                )
            }

            Err(e) => {
                eprintln!(
                    "{} {}",
                    style("Error:").red().bold(),
                    format!(
                        "Cannot stop proxy {} (platform: {}) on port {}. Please check log for details.",
                        current_proxy, proxy_config.platform, proxy_config.port
                    )
                );
            }
        }
    } else {
        println!(
            "{} {}",
            style("Info:").blue().bold(),
            format!(
                "Proxy {} (platform: {}) not run",
                current_proxy, proxy_config.platform
            )
        )
    }
}
