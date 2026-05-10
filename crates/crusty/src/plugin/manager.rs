use std::{collections::HashMap, path::Path};

use abi_stable::std_types::RString;
use crusty_plugin::bridge::{ChatCallback, PluginRef};

use crate::{config::plugin::PluginConfig, plugin::loader::load_plugin};

const HOST_CALLBACK: ChatCallback = ChatCallback {
    ask: host_ask_handler,
};

pub struct PluginManager<'a> {
    loaded_plugins: HashMap<String, (PluginRef, &'a PluginConfig)>,
    chat_hooks: HashMap<String, PluginRef>,
}

impl<'a> PluginManager<'a> {
    pub fn new() -> Self {
        Self {
            loaded_plugins: HashMap::new(),
            chat_hooks: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, id: String, config: &'a PluginConfig) {
        let plugin = load_plugin(Path::new(&config.file));
        self.loaded_plugins.insert(id, (plugin, config));
    }

    pub fn load_all(&mut self, plugins: &'a Vec<PluginConfig>) {
        for p in plugins {
            let id = format!("{}", p.id);
            self.register_plugin(id, p);
        }
    }

    pub fn run_all(&mut self) {
        let ids: Vec<String> = self.loaded_plugins.keys().cloned().collect();

        for id in ids {
            let (plugin_ref, config) = &self.loaded_plugins[&id];
            let Some(features) = config.features.as_ref() else {
                continue;
            };

            if features.contains(&format!("chat")) {
                if let Some(Some(init_chat_fn)) = plugin_ref.init_chat() {
                    let hook_id = init_chat_fn(HOST_CALLBACK);
                    self.chat_hooks.insert(hook_id.to_string(), *plugin_ref);
                }
            }
        }
    }
}

extern "C" fn host_ask_handler(plugin_id: RString, question: RString) {
    println!("🤖 [HOST] Plugin '{}' đang hỏi: {}", plugin_id, question);
}
