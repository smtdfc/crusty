use console::Term;
use dialoguer::theme::ColorfulTheme;

use crate::{
    cli::{chat::handle_chat_start, config::handle_config, utils::get_active_proxy},
    config::config::AppConfig,
    helpers::tui::{print_banner, print_error, show_loading, show_menu},
    plugin::manager::PluginManager,
};

pub async fn handle_start() {
    show_loading("Preparing ...");
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            print_error(&format!("{}", e));
            return;
        }
    };
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy(&config, "start") else {
        return;
    };

    let mut plugin_manager = PluginManager::new();
    let is_proxy_online = proxy.is_running();
    let theme = ColorfulTheme::default();
    let term = Term::stdout();

    let Some(model_name) = proxy_config.current_model.clone() else {
        print_error("No model select. Please select a model to start chat.");
        return;
    };

    if !is_proxy_online {
        print_error(
            format!(
                "Proxy {} (platform: {}) is offline. Please run proxy before.",
                current_proxy, proxy_config.platform
            )
            .as_str(),
        );
        return;
    }

    plugin_manager.load_all(&config.plugins);
    plugin_manager.run_all();

    print_banner(
        &model_name,
        &current_proxy,
        &proxy_config.platform,
        &proxy_config.host,
        proxy_config.port,
        is_proxy_online,
    );

    loop {
        let Some(opt) = show_menu(vec!["Chat", "Config"], "Select your option") else {
            break;
        };

        if opt == 0 {
            handle_chat_start(false).await;
        } else if opt == 1 {
            handle_config();
        }
    }
}
