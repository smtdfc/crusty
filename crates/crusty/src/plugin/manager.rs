use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, LazyLock, OnceLock, RwLock},
    time::Duration,
};

use abi_stable::std_types::RString;
use crusty_plugin::bridge::{ChatCallback, PluginRef};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::{
    agent::{
        agent::{AnyAgent, create_chat_agent},
        memory::session::{Session, create_session},
        message::ChatMessage,
    },
    cli::utils::{get_active_proxy_and_check, get_agent_params, get_initialized_store},
    config::plugin::PluginConfig,
    helpers::types::{ArcMutex, LazyCacheLock, LazyRwLock},
    plugin::loader::load_plugin,
};
use moka::future::Cache;

const HOST_CALLBACK: ChatCallback = ChatCallback {
    ask: host_ask_handler,
};

static AGENT_SESSIONS: LazyCacheLock<String, Box<dyn AnyAgent>> = LazyLock::new(|| {
    Cache::builder()
        .time_to_idle(Duration::from_secs(30 * 60))
        .max_capacity(1000)
        .build()
});

static LOADED_PLUGINS: LazyRwLock<HashMap<String, (PluginRef, &'static PluginConfig)>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static PLUGIN_CHAT_HOOKS: LazyRwLock<HashMap<String, PluginRef>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static HOST_RUNTIME: OnceLock<tokio::runtime::Handle> = OnceLock::new();

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
    // Capture the host's tokio runtime handle so that FFI callbacks can spawn tasks
    let _ = HOST_RUNTIME.set(tokio::runtime::Handle::current());

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

    let p_id = plugin_id.to_string();
    let s_id = session_id.to_string();
    let q = question.to_string();

    if let Some(rt) = HOST_RUNTIME.get() {
        rt.spawn(async move {
            let (proxy_config, model_name, api_key) = {
                let Some((_current_proxy, proxy_config, _proxy)) =
                    get_active_proxy_and_check("start", false)
                else {
                    error!("Failed to get active proxy for plugin chat");
                    return;
                };

                let Some((model_name, api_key)) = get_agent_params(&proxy_config) else {
                    error!("Failed to get agent parameters for plugin chat");
                    return;
                };
                (proxy_config, model_name, api_key)
            };

            let agent_arc = AGENT_SESSIONS
                .get_with(s_id.clone(), async move {
                    let agent = create_chat_agent(proxy_config.port, &api_key, &model_name);
                    Arc::new(Mutex::new(agent)) as ArcMutex<Box<dyn AnyAgent>>
                })
                .await;

            let Some(memory_store) = get_initialized_store().await else {
                error!("Failed to initialize store for plugin chat");
                return;
            };

            let mut session = match Session::load(s_id.clone(), &memory_store).await {
                Ok(s) => s,
                Err(_) => match create_session(memory_store.clone()).await {
                    Ok(mut s) => {
                        s.session_id = s_id.clone();
                        s
                    }
                    Err(e) => {
                        error!(error = ?e, "Failed to create session");
                        return;
                    }
                },
            };

            let mut agent = agent_arc.lock().await;

            let full_response = Arc::new(std::sync::Mutex::new(String::new()));
            let response_clone = Arc::clone(&full_response);

            let callback = Box::new(move |m: ChatMessage| {
                let ChatMessage::TextMessage(chunk) = m;
                let mut resp = response_clone.lock().unwrap();
                resp.push_str(&chunk.content);
            });

            if let Err(e) = agent.chat(&q, &mut session, callback).await {
                error!(error = ?e, "Failed to chat via plugin");
                return;
            }

            let final_message = full_response.lock().unwrap().clone();
            let plugins = PLUGIN_CHAT_HOOKS.read().unwrap();
            if let Some(plugin_ref) = plugins.get(&p_id) {
                if let Some(Some(handle_chat_respond)) = plugin_ref.handle_chat_respond() {
                    handle_chat_respond(s_id.clone().into(), final_message.into());
                }
            }
        });
    } else {
        error!("Host tokio runtime not available for plugin callback");
    }
}
