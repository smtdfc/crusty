use clap::Subcommand;
use tracing::error;

use crate::{
    config::config::AppConfig,
    helpers::tui::{print_error, print_success},
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
