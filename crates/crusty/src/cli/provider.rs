use clap::Subcommand;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use tracing::error;

use crate::config::config::AppConfig;
use crate::config::provider::ProviderConfig;
use crate::helpers::tui::{print_error, print_info, print_success};

#[derive(Subcommand)]
pub enum ProviderCommands {
    /// List all configured providers
    List,
    /// Add a new OpenAI-compatible provider
    Add,
    /// Remove a provider
    Remove,
    /// Switch to a different provider
    Switch,
}

pub fn handle_provider_list() {
    match AppConfig::load() {
        Ok(config) => {
            let providers = config.list_providers();

            if providers.is_empty() {
                print_info("No providers configured yet.");
                return;
            }

            println!("\nConfigured Providers:");
            println!("{:<20} {:<20} {:<50}", "Name", "Type", "Base URL");
            println!("{}", "-".repeat(90));

            for provider_name in providers {
                if let Some(provider) = config.find_provider_by_id(&provider_name) {
                    let is_active = config
                        .current_provider
                        .as_ref()
                        .map(|p| p == &provider_name)
                        .unwrap_or(false);
                    let marker = if is_active { " ➜ " } else { "   " };
                    println!(
                        "{}{:<17} {:<20} {:<50}",
                        marker, provider_name, provider.provider_type, provider.base_url
                    );
                }
            }

            if let Some(current) = config.current_provider {
                println!("\nCurrent Provider: {}", current);
            }
        }
        Err(e) => {
            error!(?e, "Failed to load config");
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_provider_add() {
    let theme = ColorfulTheme::default();

    let name: String = Input::with_theme(&theme)
        .with_prompt("Provider name")
        .interact_text()
        .unwrap_or_default();

    if name.is_empty() {
        print_error("Provider name cannot be empty");
        return;
    }

    let providers_types = vec!["openai", "anthropic", "custom"];
    let provider_type_idx = Select::with_theme(&theme)
        .with_prompt("Select provider type")
        .items(&providers_types)
        .interact()
        .unwrap_or(0);

    let provider_type = providers_types[provider_type_idx];

    let base_url: String = Input::with_theme(&theme)
        .with_prompt("Base URL (e.g., https://api.openai.com/v1)")
        .interact_text()
        .unwrap_or_default();

    if base_url.is_empty() {
        print_error("Base URL cannot be empty");
        return;
    }

    let api_key: String = Input::with_theme(&theme)
        .with_prompt("API Key")
        .interact_text()
        .unwrap_or_default();

    if api_key.is_empty() {
        print_error("API Key cannot be empty");
        return;
    }

    let default_model: String = Input::with_theme(&theme)
        .with_prompt("Default model (required)")
        .interact_text()
        .unwrap_or_default();

    if default_model.is_empty() {
        print_error("Default model cannot be empty");
        return;
    }

    let provider_config =
        ProviderConfig::new(provider_type, &base_url, &api_key, Some(default_model));

    match AppConfig::load() {
        Ok(mut config) => {
            if let Err(e) = config.add_provider(name.clone(), provider_config) {
                error!(?e, "Failed to add provider");
                print_error(&format!("Failed to add provider: {}", e));
                return;
            }

            if let Err(e) = config.save() {
                error!(?e, "Failed to save config");
                print_error(&format!("Failed to save config: {}", e));
                return;
            }

            print_success(&format!("Provider '{}' added successfully", name));
        }
        Err(e) => {
            error!(?e, "Failed to load config");
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_provider_remove() {
    let theme = ColorfulTheme::default();

    match AppConfig::load() {
        Ok(config) => {
            let providers = config.list_providers();

            if providers.is_empty() {
                print_info("No providers configured.");
                return;
            }

            let idx = Select::with_theme(&theme)
                .with_prompt("Select provider to remove")
                .items(&providers)
                .interact()
                .unwrap_or(providers.len());

            if idx >= providers.len() {
                return;
            }

            let provider_name = &providers[idx];

            let confirm = Confirm::with_theme(&theme)
                .with_prompt(&format!("Remove provider '{}'?", provider_name))
                .default(false)
                .interact()
                .unwrap_or(false);

            if !confirm {
                return;
            }

            let mut config = AppConfig::load().unwrap();
            if let Err(e) = config.remove_provider(provider_name) {
                error!(?e, "Failed to remove provider");
                print_error(&format!("Failed to remove provider: {}", e));
                return;
            }

            if let Err(e) = config.save() {
                error!(?e, "Failed to save config");
                print_error(&format!("Failed to save config: {}", e));
                return;
            }

            print_success(&format!(
                "Provider '{}' removed successfully",
                provider_name
            ));
        }
        Err(e) => {
            error!(?e, "Failed to load config");
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_provider_switch() {
    let theme = ColorfulTheme::default();

    match AppConfig::load() {
        Ok(config) => {
            let providers = config.list_providers();

            if providers.is_empty() {
                print_info("No providers configured. Run 'crusty provider add' first.");
                return;
            }

            let idx = Select::with_theme(&theme)
                .with_prompt("Select provider to activate")
                .items(&providers)
                .interact()
                .unwrap_or(providers.len());

            if idx >= providers.len() {
                return;
            }

            let provider_name = providers[idx].clone();
            let mut config = AppConfig::load().unwrap();

            if let Err(e) = config.set_current_provider(Some(provider_name.clone())) {
                error!(?e, "Failed to switch provider");
                print_error(&format!("Failed to switch provider: {}", e));
                return;
            }

            if let Err(e) = config.save() {
                error!(?e, "Failed to save config");
                print_error(&format!("Failed to save config: {}", e));
                return;
            }

            print_success(&format!("Switched to provider '{}'", provider_name));
        }
        Err(e) => {
            error!(?e, "Failed to load config");
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}
