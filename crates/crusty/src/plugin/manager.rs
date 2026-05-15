use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, LazyLock, RwLock},
    time::Duration,
};

use abi_stable::std_types::RString;
use crusty_plugin::bridge::{ChatCallback, PluginRef};
use tracing::{error, info};

use crate::{
    agent::{
        agent::{AnyAgent, create_chat_agent},
        memory::store::get_store,
    },
    cli::utils::get_active_proxy,
    config::{config::GLOBAL_CONFIG, plugin::PluginConfig},
    helpers::tui::print_error,
    plugin::loader::load_plugin,
};
use moka::future::Cache;

const HOST_CALLBACK: ChatCallback = ChatCallback {
    ask: host_ask_handler,
};

static AGENT_SESSIONS: LazyLock<Cache<String, Arc<dyn AnyAgent>>> = LazyLock::new(|| {
    Cache::builder()
        .time_to_idle(Duration::from_secs(30 * 60))
        .max_capacity(1000)
        .build()
});

static LOADED_PLUGINS: LazyLock<RwLock<HashMap<String, (PluginRef, &'static PluginConfig)>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static PLUGIN_CHAT_HOOKS: LazyLock<RwLock<HashMap<String, PluginRef>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn register_plugin(id: &str, config: &'static PluginConfig) {
    let mut plugins = LOADED_PLUGINS.write().unwrap();
    let plugin = load_plugin(Path::new(&config.file));
    plugins.insert(id.to_string(), (plugin, config));
    info!("Register plugin {}", config.id);
}

pub fn load_all_plugin(plugins: &'static Vec<PluginConfig>) {
    for p in plugins {
        let id = format!("{}", p.id);
        register_plugin(&id, p);
    }
}

pub fn run_all_plugin() {
    let plugins = LOADED_PLUGINS.read().unwrap();
    let mut chat_hooks = PLUGIN_CHAT_HOOKS.write().unwrap();
    let ids: Vec<String> = plugins.keys().cloned().collect();

    for id in ids {
        let (plugin_ref, config) = &plugins[&id];
        let Some(features) = config.features.as_ref() else {
            continue;
        };

        if features.contains(&format!("chat")) {
            if let Some(Some(init_chat_fn)) = plugin_ref.init_chat() {
                let hook_id = init_chat_fn(HOST_CALLBACK);
                chat_hooks.insert(hook_id.to_string(), *plugin_ref);
            }
        }

        info!("Started plugin {}", config.id);
    }
}

extern "C" fn host_ask_handler(plugin_id: RString, session_id: RString, question: RString) {
    info!(
        "received message from plugin {} (session ID: {})",
        plugin_id, session_id
    );

    let s_id = session_id.to_string();

    let agent = AGENT_SESSIONS.get_with(s_id.clone(), async move {
        let config = GLOBAL_CONFIG.read().unwrap();
        let Some((_current_proxy, proxy_config, _proxy)) = get_active_proxy(&config, "start")
        else {
            panic!("")
        };

        let Some(model_name) = proxy_config.current_model.clone() else {
            print_error("No model select. Please select a model to start chat.");
            panic!("")
        };

        let api_key = match proxy_config.api_key.as_deref() {
            None => String::from(""),
            Some(v) => v.to_string(),
        };

        let Some(ref store_config) = config.store else {
            print_error("Store not configured. Please setup your store.");
            panic!("")
        };
        let memory_store = match get_store(store_config).await {
            Ok(s) => s,
            Err(e) => {
                error!(error = ?e, "Failed to create session");
                print_error(&format!("Cannot init chat session now. Cause: {}", e));
                panic!("")
            }
        };

        let mut agent = create_chat_agent(proxy_config.port, &api_key, &model_name);
        Arc::from(agent) as Arc<dyn AnyAgent>
    });
}
