use dialoguer::{Confirm, Input, theme::ColorfulTheme};
use tracing::error;

use crate::{
    config::{config::AppConfig, store::StoreConfig},
    exceptions::crusty::CrustyError,
    helpers::tui::{print_error, print_info, print_success, print_warning},
};

fn load_store_config() -> Result<AppConfig, CrustyError> {
    AppConfig::load()
}

fn default_store_path(config: &AppConfig) -> String {
    config
        .store
        .as_ref()
        .map(|store| extract_store_path(&store.uri))
        .unwrap_or_else(|| {
            let mut path = AppConfig::get_data_dir();
            path.push("store.db");
            path.to_string_lossy().replace('\\', "/")
        })
}

fn extract_store_path(uri: &str) -> String {
    uri.strip_prefix("sqlite:")
        .and_then(|value| value.strip_suffix("?mode=rwc"))
        .unwrap_or(uri)
        .to_string()
}

fn build_sqlite_store_uri(path: &str) -> String {
    format!("sqlite:{}?mode=rwc", path.trim().replace('\\', "/"))
}

fn build_store_config(path: String) -> StoreConfig {
    StoreConfig {
        store_type: String::from("sqlite"),
        uri: build_sqlite_store_uri(&path),
    }
}

pub fn handle_store_show() {
    match load_store_config() {
        Ok(config) => {
            println!("\nStore settings");
            println!("{}", "-".repeat(24));

            match config.store {
                Some(store) => {
                    print_info(&format!("Type: {}", store.store_type));
                    print_info(&format!("URI : {}", store.uri));
                    print_info(&format!("Path: {}", extract_store_path(&store.uri)));
                }
                None => print_warning("Store is not configured"),
            }
        }
        Err(e) => {
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_store_edit() {
    let theme = ColorfulTheme::default();

    match load_store_config() {
        Ok(mut config) => {
            let default_path = default_store_path(&config);
            let store_path: String = Input::with_theme(&theme)
                .with_prompt("Store database path")
                .with_initial_text(default_path)
                .interact_text()
                .unwrap_or_default();

            if store_path.trim().is_empty() {
                print_error("Store path cannot be empty");
                return;
            }

            let confirm = Confirm::with_theme(&theme)
                .with_prompt(&format!("Save store path '{}' ?", store_path))
                .default(true)
                .interact()
                .unwrap_or(true);

            if !confirm {
                return;
            }

            config.store = Some(build_store_config(store_path));

            if let Err(e) = config.save() {
                error!(error = ?e, "Failed to save store config");
                print_error(&format!("Failed to save config: {}", e));
                return;
            }

            print_success("Store configuration updated successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}

pub fn handle_store_reset() {
    let theme = ColorfulTheme::default();

    match load_store_config() {
        Ok(mut config) => {
            let mut default_path = AppConfig::get_data_dir();
            default_path.push("store.db");
            let default_path = default_path.to_string_lossy().replace('\\', "/");

            let confirm = Confirm::with_theme(&theme)
                .with_prompt(&format!("Reset store path to '{}' ?", default_path))
                .default(true)
                .interact()
                .unwrap_or(true);

            if !confirm {
                return;
            }

            config.store = Some(build_store_config(default_path));

            if let Err(e) = config.save() {
                error!(error = ?e, "Failed to save store config");
                print_error(&format!("Failed to save config: {}", e));
                return;
            }

            print_success("Store configuration reset successfully");
        }
        Err(e) => {
            print_error(&format!("Failed to load config: {}", e));
        }
    }
}