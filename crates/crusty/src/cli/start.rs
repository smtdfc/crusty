// use console::Term;
// use dialoguer::theme::ColorfulTheme;

use tracing::error;

use crate::{
    agent::memory::{session::create_session, store::get_store},
    cli::{chat::handle_chat_start, config::handle_config, utils::get_active_proxy},
    config::{config::GLOBAL_CONFIG, plugin::PluginConfig},
    helpers::tui::{print_banner, print_error, show_loading, show_menu},
    plugin::manager::{load_all_plugin, run_all_plugin},
};

pub async fn handle_start(jump_to_chat: bool) {
    show_loading("Preparing ...");
    let config = GLOBAL_CONFIG.read().unwrap();
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy(&config, "start") else {
        return;
    };

    let mut is_proxy_online = false;
    // let theme = ColorfulTheme::default();
    // let term = Term::stdout();

    let Some(model_name) = proxy_config.current_model.clone() else {
        print_error("No model select. Please select a model to start chat.");
        return;
    };

    match proxy.is_running() {
        Ok(false) => {
            print_error(&format!(
                "Proxy {} (platform: {}) is offline. Please run proxy before.",
                current_proxy, proxy_config.platform
            ));

            return;
        }

        Err(e) => {
            error!(error = ?e, "Failed to check proxy status");
            print_error(&format!("Failed to check proxy. Please try again"));
            return;
        }

        Ok(true) => is_proxy_online = true,
    }

    // load and start all plugin
    let plugins = config.plugins.clone();
    let plugins_static: &'static Vec<PluginConfig> = Box::leak(Box::new(plugins));
    load_all_plugin(&plugins_static);
    run_all_plugin();

    let Some(ref store_config) = config.store else {
        print_error("Store not configured. Please setup your store.");
        return;
    };
    let memory_store = match get_store(store_config).await {
        Ok(s) => s,
        Err(e) => {
            error!(error = ?e, "Failed to create session");
            print_error(&format!("Cannot init chat session now. Cause: {}", e));
            return;
        }
    };

    let session = match create_session(&memory_store).await {
        Ok(s) => s,
        Err(e) => {
            error!(error = ?e, "Failed to create session");
            print_error(&format!("Cannot init chat session now. Cause: {}", e));
            return;
        }
    };

    print_banner(
        &model_name,
        &current_proxy,
        &proxy_config.platform,
        &proxy_config.host,
        proxy_config.port,
        is_proxy_online,
        &session.session_id,
    );

    if jump_to_chat {
        handle_chat_start(&memory_store).await;
        return;
    }

    loop {
        let Some(opt) = show_menu(vec!["Chat", "Config"], "Select your option") else {
            break;
        };

        if opt == 0 {
            handle_chat_start(&memory_store).await;
        } else if opt == 1 {
            handle_config();
        }
    }
}
