// use console::Term;
// use dialoguer::theme::ColorfulTheme;


use tracing::error;

use crate::{
    agent::memory::session::create_session,
    cli::{
        chat::handle_chat_start,
        config::handle_config,
        utils::{get_active_proxy_and_check, get_agent_params, get_initialized_store},
    },
    config::{config::GLOBAL_CONFIG, plugin::PluginConfig},
    helpers::tui::{print_banner, print_error, show_loading, show_menu},
    plugin::manager::{load_all_plugin, run_all_plugin},
};

pub async fn handle_start(jump_to_chat: bool) {
    show_loading("Preparing ...");

    let Some((current_proxy, proxy_config, _proxy)) = get_active_proxy_and_check("start", true)
    else {
        return;
    };

    let Some((model_name, _api_key)) = get_agent_params(&proxy_config) else {
        return;
    };

    // load and start all plugin
    let plugins = {
        let config = GLOBAL_CONFIG.read().unwrap();
        config.plugins.clone()
    };
    let plugins_static: &'static Vec<PluginConfig> = Box::leak(Box::new(plugins));
    load_all_plugin(&plugins_static);
    run_all_plugin();

    let Some(memory_store) = get_initialized_store().await else {
        return;
    };

    let session = match create_session(memory_store.clone()).await {
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
        true, // is_proxy_online
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
