use crate::{config::config::GLOBAL_CONFIG, helpers::tui::show_loading};

pub fn handle_config() {
    show_loading("Preparing ...");
    let config = GLOBAL_CONFIG.read().unwrap();
}
