// use console::Term;
// use dialoguer::theme::ColorfulTheme;


use tracing::error;

use crate::{
    agent::memory::session::create_session,
    cli::{
        chat::handle_chat_start,
        config::handle_config,
        utils::{get_active_proxy_and_check, get_active_provider_and_check, get_agent_params},
    },
    config::{config::{AppConfig, RunMode, GLOBAL_CONFIG}, plugin::PluginConfig},
    helpers::tui::{print_banner, print_error, show_loading, show_menu},
    plugin::manager::{load_all_plugin, run_all_plugin},
};

pub async fn handle_start(jump_to_chat: bool) {
    show_loading("Preparing ...");

    // Load config to check current mode
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(?e, "Failed to load config");
            print_error(&format!("Failed to load config: {}", e));
            return;
        }
    };

    let current_mode = match config.get_mode() {
        Ok(mode) => mode,
        Err(e) => {
            error!(?e, "No mode selected");
            print_error(&format!("{}", e));
            return;
        }
    };

    // Get provider or proxy info based on selected mode
    let (provider_info, proxy_info, model_name) = match current_mode {
        RunMode::Provider => {
            // Must use provider in provider mode
            match get_active_provider_and_check() {
                Some((provider_name, provider_config)) => {
                    let model = provider_config
                        .default_model
                        .clone()
                        .unwrap_or_else(|| "gpt-3.5-turbo".to_string());
                    (
                        Some((provider_name, provider_config)),
                        None,
                        model,
                    )
                }
                None => {
                    error!("Provider mode selected but no active provider configured");
                    print_error("Provider mode is active but no provider is configured. Please run 'crusty provider add' to add a provider.");
                    return;
                }
            }
        }
        RunMode::Proxy => {
            // Must use proxy in proxy mode
            match get_active_proxy_and_check("start", true) {
                Some((current_proxy, proxy_config, _proxy)) => {
                    match get_agent_params(&proxy_config) {
                        Some((model_name, _api_key)) => {
                            (None, Some((current_proxy, proxy_config)), model_name)
                        }
                        None => {
                            error!("Failed to get agent parameters");
                            return;
                        }
                    }
                }
                None => {
                    error!("Proxy mode selected but no active proxy configured");
                    print_error("Proxy mode is active but no proxy is configured. Please run 'crusty setup' to configure a proxy.");
                    return;
                }
            }
        }
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

    // Print banner with appropriate info
    if let Some((provider_name, provider_config)) = &provider_info {
        print_banner(
            &model_name,
            provider_name,
            &provider_config.provider_type,
            "api",
            0,
            true,
            &session.session_id,
        );
    } else if let Some((current_proxy, proxy_config)) = &proxy_info {
        print_banner(
            &model_name,
            current_proxy,
            &proxy_config.platform,
            &proxy_config.host,
            proxy_config.port,
            true,
            &session.session_id,
        );
    }

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

use crate::agent::memory::store::{SharedMemoryStore, get_store};

pub async fn get_initialized_store() -> Option<SharedMemoryStore> {
    let store_config = {
        let config = GLOBAL_CONFIG.read().unwrap();
        config.store.clone()
    };

    let Some(store_config) = store_config else {
        print_error("Store not configured. Please setup your store.");
        return None;
    };

    match get_store(&store_config).await {
        Ok(s) => Some(s),
        Err(e) => {
            error!(error = ?e, "Failed to create store");
            print_error(&format!("Cannot init chat session now. Cause: {}", e));
            None
        }
    }
}
