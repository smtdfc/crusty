use clap::Subcommand;
use dialoguer::{Confirm, Select, theme::ColorfulTheme};
use tracing::error;

use crate::{
    config::config::AppConfig,
    helpers::tui::{print_error, print_success, print_warning},
    plugin::loader::install_plugin,
};

#[derive(Subcommand)]
pub enum PluginCommands {
    /// Install a plugin from a local package or manifest path
    Install {
        /// Path to the plugin package or manifest file
        #[arg(short, long)]
        path: String,
    },
}

fn load_plugin_config() -> Result<AppConfig, String> {
    AppConfig::load().map_err(|e| format!("{}", e))
}

fn plugin_root_path(plugin_id: &str) -> std::path::PathBuf {
    let mut dest_path = AppConfig::get_data_dir();
    for part in plugin_id.split('/') {
        dest_path.push(part);
    }
    dest_path
}

fn remove_plugin_assets(plugin_id: &str) -> Result<(), String> {
    let dest_path = plugin_root_path(plugin_id);
    if dest_path.exists() {
        std::fs::remove_dir_all(&dest_path).map_err(|e| {
            format!(
                "Failed to remove installed plugin files at {:?}. Cause: {}",
                dest_path, e
            )
        })?;
    }

    Ok(())
}

fn print_plugin_table_header() {
    println!("\nInstalled Plugins:");
    println!("{:<24} {:<24} {:<48}", "Name", "ID", "Binary");
    println!("{}", "-".repeat(100));
}

fn print_plugin_row(name: &str, id: &str, file: &str) {
    println!("{:<24} {:<24} {:<48}", name, id, file);
}

pub fn handle_plugin_install(path: &str) {
    let mut config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(error = ?e, "Failed to load config");
            print_error(&format!("Failed to load config"));

            return;
        }
    };

    match install_plugin(path, &mut config) {
        Ok(_) => match config.save() {
            Ok(_) => print_success("Plugin installed"),
            Err(msg) => print_error(&format!("{}", msg)),
        },
        Err(msg) => print_error(&format!("{}", msg)),
    }
}

pub fn handle_plugin_list() {
    match load_plugin_config() {
        Ok(config) => {
            if config.plugins.is_empty() {
                print_warning("No plugins installed");
                return;
            }

            print_plugin_table_header();
            for plugin in config.plugins {
                print_plugin_row(&plugin.name, &plugin.id, &plugin.file);
            }
        }
        Err(msg) => print_error(&format!("Failed to load config: {}", msg)),
    }
}

pub fn handle_plugin_remove() {
    let theme = ColorfulTheme::default();

    match load_plugin_config() {
        Ok(mut config) => {
            if config.plugins.is_empty() {
                print_warning("No plugins installed");
                return;
            }

            let plugin_names: Vec<String> = config
                .plugins
                .iter()
                .map(|plugin| format!("{} ({})", plugin.name, plugin.id))
                .collect();

            let idx = Select::with_theme(&theme)
                .with_prompt("Select plugin to remove")
                .items(&plugin_names)
                .interact()
                .unwrap_or(plugin_names.len());

            if idx >= plugin_names.len() {
                return;
            }

            let plugin = config.plugins[idx].clone();
            let confirm = Confirm::with_theme(&theme)
                .with_prompt(&format!("Remove plugin '{}' ?", plugin.name))
                .default(false)
                .interact()
                .unwrap_or(false);

            if !confirm {
                return;
            }

            if let Err(msg) = remove_plugin_assets(&plugin.id) {
                print_error(&msg);
                return;
            }

            config.plugins.retain(|item| item.id != plugin.id);

            if let Err(msg) = config.save() {
                print_error(&format!("Failed to save config: {}", msg));
                return;
            }

            print_success(&format!("Plugin '{}' removed successfully", plugin.name));
        }
        Err(msg) => print_error(&format!("Failed to load config: {}", msg)),
    }
}
