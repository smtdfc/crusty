use clap::Subcommand;
use dialoguer::{theme::ColorfulTheme, Select};
use tracing::error;

use crate::config::config::{AppConfig, RunMode};
use crate::helpers::tui::{print_error, print_info, print_success};

#[derive(Subcommand)]
pub enum ModeCommands {
    /// Show current mode
    Show,
    /// Switch between proxy and provider mode
    Switch,
}

pub fn handle_mode_show() {
    match AppConfig::load() {
        Ok(config) => {
            match config.get_mode() {
                Ok(current_mode) => {
                    println!("\n📍 Current Mode: {}", current_mode);

                    match current_mode {
                        RunMode::Proxy => {
                            if let Some(ref proxy_name) = config.current_proxy {
                                println!("   Active Proxy: {}", proxy_name);
                            }
                            println!("   Available Proxies: {}", config.ai_proxies.keys()
                                .cloned()
                                .collect::<Vec<_>>()
                                .join(", "));
                        }
                        RunMode::Provider => {
                            if let Some(ref provider_name) = config.current_provider {
                                println!("   Active Provider: {}", provider_name);
                            }
                            println!(
                                "   Available Providers: {}",
                                config.list_providers().join(", ")
                            );
                        }
                    }
                }
                Err(e) => {
                    error!(?e, "No mode selected");
                    print_error(&format!("{}", e));
                }
            }
        }
        Err(e) => {
            error!(?e, "Failed to load config");
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_mode_switch() {
    match AppConfig::load() {
        Ok(mut config) => {
            let theme = ColorfulTheme::default();
            let modes = vec!["Provider", "Proxy"];

            // Get current mode if set
            let (current_mode, selected_idx) = match config.get_mode() {
                Ok(mode) => {
                    let idx = match mode {
                        RunMode::Provider => 0,
                        RunMode::Proxy => 1,
                    };
                    (Some(mode), idx)
                }
                Err(_) => {
                    // No mode set yet, ask user to choose
                    (None, 0)
                }
            };

            println!("\nSelect mode:");
            let idx = Select::with_theme(&theme)
                .items(&modes)
                .default(selected_idx)
                .interact()
                .unwrap_or(0);

            let new_mode = match idx {
                0 => RunMode::Provider,
                _ => RunMode::Proxy,
            };

            // Check if trying to switch to the same mode
            if let Some(current) = current_mode {
                if current == new_mode {
                    print_info(&format!("Already in {} mode", new_mode));
                    return;
                }
            }

            // Switch mode
            if let Err(e) = config.set_mode(new_mode) {
                error!(?e, "Failed to switch mode");
                print_error(&format!("Cannot switch to {} mode: {}", new_mode, e));
                return;
            }

            // Save configuration
            if let Err(e) = config.save() {
                error!(?e, "Failed to save config");
                print_error(&format!("Failed to save config: {}", e));
                return;
            }

            print_success(&format!("Switched to {} mode", new_mode));

            // Show what's active in the new mode
            match new_mode {
                RunMode::Proxy => {
                    if let Some(ref proxy_name) = config.current_proxy {
                        print_info(&format!("Active proxy: {}", proxy_name));
                    }
                }
                RunMode::Provider => {
                    if let Some(ref provider_name) = config.current_provider {
                        print_info(&format!("Active provider: {}", provider_name));
                    }
                }
            }
        }
        Err(e) => {
            error!(?e, "Failed to load config");
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}
