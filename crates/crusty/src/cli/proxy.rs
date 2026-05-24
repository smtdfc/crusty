use clap::Subcommand;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use tracing::error;

use crate::{
    cli::utils::get_active_proxy_and_check,
    config::{ai_proxy::AIProxyConfig, config::AppConfig},
    exceptions::crusty::CrustyError,
    helpers::tui::{print_error, print_info, print_success, print_warning, show_loading},
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

fn prompt_9router_proxy_config(
    existing: Option<&AIProxyConfig>,
) -> Result<AIProxyConfig, CrustyError> {
    let theme = ColorfulTheme::default();

    let is_local_default = existing.map(|proxy| proxy.is_local).unwrap_or(true);
    let is_local = Confirm::with_theme(&theme)
        .with_prompt("Use proxy locally?")
        .default(is_local_default)
        .interact()
        .unwrap_or(is_local_default);

    let default_host = if is_local {
        String::from("localhost")
    } else {
        existing.map(|proxy| proxy.host.clone()).unwrap_or_default()
    };

    let host = if is_local {
        String::from("localhost")
    } else {
        Input::with_theme(&theme)
            .with_prompt("Proxy host")
            .with_initial_text(default_host)
            .interact_text()
            .unwrap_or_default()
    };

    let port = if is_local {
        let port_default = existing
            .map(|proxy| proxy.port.to_string())
            .unwrap_or_else(|| String::from("8000"));
        let port_input: String = Input::with_theme(&theme)
            .with_prompt("Proxy port")
            .with_initial_text(port_default)
            .interact_text()
            .unwrap_or_default();
        port_input.trim().parse::<u64>().map_err(|e| {
            CrustyError::ConfigError(format!("Invalid proxy port '{}': {}", port_input, e))
        })?
    } else {
        existing.map(|proxy| proxy.port).unwrap_or(8000)
    };

    let api_key_default = existing
        .and_then(|proxy| proxy.api_key.clone())
        .unwrap_or_default();
    let api_key_input: String = Input::with_theme(&theme)
        .with_prompt("API key (optional)")
        .with_initial_text(api_key_default)
        .interact_text()
        .unwrap_or_default();
    let api_key = if api_key_input.trim().is_empty() {
        None
    } else {
        Some(api_key_input)
    };

    let model_default = existing
        .and_then(|proxy| proxy.current_model.clone())
        .unwrap_or_else(|| String::from("crusty_combo"));
    let current_model: String = Input::with_theme(&theme)
        .with_prompt("Current model")
        .with_initial_text(model_default)
        .interact_text()
        .unwrap_or_default();

    if current_model.trim().is_empty() {
        return Err(CrustyError::ConfigError(
            "Current model cannot be empty".into(),
        ));
    }

    Ok(AIProxyConfig {
        platform: String::from("9router"),
        is_local,
        host,
        port,
        api_key,
        current_model: Some(current_model),
    })
}

fn load_proxy_config() -> Result<AppConfig, CrustyError> {
    AppConfig::load()
}

fn print_proxy_table_header() {
    println!("\nConfigured Proxies:");
    println!(
        "{:<20} {:<16} {:<18} {:<10} {:<16}",
        "Name", "Platform", "Host", "Port", "Model"
    );
    println!("{}", "-".repeat(82));
}

fn print_proxy_row(name: &str, proxy: &AIProxyConfig, is_current: bool) {
    let marker = if is_current { " ➜ " } else { "   " };
    println!(
        "{}{:<17} {:<16} {:<18} {:<10} {:<16}",
        marker,
        name,
        proxy.platform,
        proxy.host,
        proxy.port,
        proxy.current_model.clone().unwrap_or_default()
    );
}

fn choose_proxy_name(prompt: &str, proxies: &[String]) -> Option<usize> {
    let theme = ColorfulTheme::default();
    let idx = Select::with_theme(&theme)
        .with_prompt(prompt)
        .items(proxies)
        .interact()
        .unwrap_or(proxies.len());

    if idx >= proxies.len() {
        None
    } else {
        Some(idx)
    }
}

pub fn handle_proxy_list() {
    match load_proxy_config() {
        Ok(config) => {
            let proxies = config.list_proxies();
            if proxies.is_empty() {
                print_warning("No proxies configured");
                return;
            }

            print_proxy_table_header();

            for name in proxies {
                if let Some(proxy) = config.find_proxy_by_id(&name) {
                    print_proxy_row(
                        &name,
                        &proxy,
                        config
                            .current_proxy
                            .as_ref()
                            .map(|current| current == &name)
                            .unwrap_or(false),
                    );
                }
            }

            if let Some(current) = config.current_proxy {
                println!("\nCurrent Proxy: {}", current);
            }
        }
        Err(e) => {
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_proxy_add() {
    let theme = ColorfulTheme::default();

    let name: String = Input::with_theme(&theme)
        .with_prompt("Proxy name")
        .interact_text()
        .unwrap_or_default();

    if name.trim().is_empty() {
        print_error("Proxy name cannot be empty");
        return;
    }

    match load_proxy_config() {
        Ok(mut config) => {
            if config.ai_proxies.contains_key(&name) {
                print_error(&format!("Proxy '{}' already exists", name));
                return;
            }

            match prompt_9router_proxy_config(None) {
                Ok(proxy_config) => {
                    if let Err(e) = config.add_proxy(name.clone(), proxy_config) {
                        print_error(&format!("Failed to add proxy: {}", e));
                        return;
                    }

                    if let Err(e) = config.save() {
                        print_error(&format!("Failed to save config: {}", e));
                        return;
                    }

                    print_success(&format!("Proxy '{}' added successfully", name));
                }
                Err(e) => {
                    print_error(&format!("Failed to prepare proxy config: {}", e));
                }
            }
        }
        Err(e) => {
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_proxy_edit() {
    let theme = ColorfulTheme::default();

    match load_proxy_config() {
        Ok(mut config) => {
            let proxies = config.list_proxies();
            if proxies.is_empty() {
                print_warning("No proxies configured");
                return;
            }

            let Some(idx) = choose_proxy_name("Select proxy to edit", &proxies) else {
                return;
            };

            let old_name = proxies[idx].clone();
            let existing = match config.find_proxy_by_id(&old_name) {
                Some(proxy) => proxy,
                None => {
                    print_error("Selected proxy was not found");
                    return;
                }
            };

            let new_name: String = Input::with_theme(&theme)
                .with_prompt("Proxy name")
                .with_initial_text(old_name.clone())
                .interact_text()
                .unwrap_or_default();

            if new_name.trim().is_empty() {
                print_error("Proxy name cannot be empty");
                return;
            }

            if new_name != old_name && config.ai_proxies.contains_key(&new_name) {
                print_error(&format!("Proxy '{}' already exists", new_name));
                return;
            }

            match prompt_9router_proxy_config(Some(&existing)) {
                Ok(proxy_config) => {
                    config.ai_proxies.remove(&old_name);
                    config.ai_proxies.insert(new_name.clone(), proxy_config);

                    if config
                        .current_proxy
                        .as_ref()
                        .map(|current| current == &old_name)
                        .unwrap_or(false)
                    {
                        config.current_proxy = Some(new_name.clone());
                    }

                    if let Err(e) = config.save() {
                        print_error(&format!("Failed to save config: {}", e));
                        return;
                    }

                    print_success(&format!("Proxy '{}' updated successfully", new_name));
                }
                Err(e) => {
                    print_error(&format!("Failed to update proxy: {}", e));
                }
            }
        }
        Err(e) => {
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_proxy_remove() {
    let theme = ColorfulTheme::default();

    match load_proxy_config() {
        Ok(mut config) => {
            let proxies = config.list_proxies();
            if proxies.is_empty() {
                print_warning("No proxies configured");
                return;
            }

            let Some(idx) = choose_proxy_name("Select proxy to remove", &proxies) else {
                return;
            };

            let proxy_name = proxies[idx].clone();
            let confirm = Confirm::with_theme(&theme)
                .with_prompt(&format!("Remove proxy '{}' ?", proxy_name))
                .default(false)
                .interact()
                .unwrap_or(false);

            if !confirm {
                return;
            }

            if let Err(e) = config.remove_proxy(&proxy_name) {
                print_error(&format!("Failed to remove proxy: {}", e));
                return;
            }

            if let Err(e) = config.save() {
                print_error(&format!("Failed to save config: {}", e));
                return;
            }

            print_success(&format!("Proxy '{}' removed successfully", proxy_name));
        }
        Err(e) => {
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_proxy_switch() {
    match load_proxy_config() {
        Ok(mut config) => {
            let proxies = config.list_proxies();
            if proxies.is_empty() {
                print_warning("No proxies configured");
                return;
            }

            let Some(idx) = choose_proxy_name("Select proxy to activate", &proxies) else {
                return;
            };

            let proxy_name = proxies[idx].clone();
            if let Err(e) = config.set_current_proxy(Some(proxy_name.clone())) {
                print_error(&format!("Failed to switch proxy: {}", e));
                return;
            }

            if let Err(e) = config.save() {
                print_error(&format!("Failed to save config: {}", e));
                return;
            }

            print_success(&format!("Switched to proxy '{}'", proxy_name));
        }
        Err(e) => {
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_proxy_start() {
    show_loading("Preparing ...");

    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy_and_check("start", false)
    else {
        return;
    };

    match proxy.is_running() {
        Ok(_t) => match proxy.start() {
            Ok(()) => {
                print_info(&format!(
                    "Proxy {} (platform: {}) is started",
                    current_proxy, proxy_config.platform
                ));
            }

            Err(e) => {
                error!(error = ?e, "Failed to start proxy");
                print_error(&format!(
                    "Cannot start proxy {} (platform: {}) on port {}. Please check log for details.",
                    current_proxy, proxy_config.platform, proxy_config.port
                ));
            }
        },

        Err(e) => {
            error!(error = ?e, "Failed to start proxy");
            print_error(&format!(
                "Cannot start proxy {} (platform: {}) on port {}. Please check log for details.",
                current_proxy, proxy_config.platform, proxy_config.port
            ));
        }
    }
}

pub fn handle_proxy_stop() {
    show_loading("Preparing ...");

    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy_and_check("stop", false)
    else {
        return;
    };

    match proxy.is_running() {
        Ok(_t) => match proxy.stop() {
            Ok(()) => {
                print_info(&format!(
                    "Proxy {} (platform: {}) is stopped",
                    current_proxy, proxy_config.platform
                ));
            }

            Err(e) => {
                error!(error = ?e, "Failed to stop proxy");
                print_error(&format!(
                    "Cannot stop proxy {} (platform: {}) on port {}. Please check log for details.",
                    current_proxy, proxy_config.platform, proxy_config.port
                ));
            }
        },

        Err(e) => {
            error!(error = ?e, "Failed to stop proxy");
            print_error(&format!(
                "Cannot stop proxy {} (platform: {}) on port {}. Please check log for details.",
                current_proxy, proxy_config.platform, proxy_config.port
            ));
        }
    }
}

pub fn handle_proxy_dashboard() {
    show_loading("Preparing ...");

    let Some((_current_proxy, _proxy_config, proxy)) =
        get_active_proxy_and_check("launch dashboard", true)
    else {
        return;
    };

    match opener::open(proxy.get_dashboard_url()) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
