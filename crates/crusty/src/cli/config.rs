use crate::{config::config::GLOBAL_CONFIG, helpers::tui::show_loading};

pub fn handle_config() {
    show_loading("Preparing ...");
    let _config = GLOBAL_CONFIG.read().unwrap();
}
