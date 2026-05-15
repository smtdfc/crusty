use std::{
    collections::HashMap,
    path::Path,
    sync::{LazyLock, RwLock},
};

use abi_stable::std_types::RString;
use crusty_plugin::bridge::{ChatCallback, PluginRef};
use tracing::info;

use crate::{config::plugin::PluginConfig, plugin::loader::load_plugin};

const HOST_CALLBACK: ChatCallback = ChatCallback {
    ask: host_ask_handler,
};

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
}
