use tracing::error;

use crate::{
    config::config::AppConfig,
    helpers::tui::{print_error, show_loading},
};

pub fn handle_config() {
    show_loading("Preparing ...");
    let _config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(error = ?e, "Failed to load config");
            print_error(&format!("Failed to load config"));

            return;
        }
    };
}
