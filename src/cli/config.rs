use console::style;

use crate::{config::config::AppConfig, helpers::tui::show_loading};

pub fn handle_config() {
    show_loading("Preparing ...");
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);

            return;
        }
    };
}
