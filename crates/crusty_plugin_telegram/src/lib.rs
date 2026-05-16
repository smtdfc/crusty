use abi_stable::export_root_module;
use abi_stable::prefix_type::PrefixTypeTrait;
use abi_stable::std_types::RString;
use crusty_plugin::bridge::ChatCallback;
use crusty_plugin::{
    bridge::{Plugin, PluginRef},
    plugin::PluginInfo,
};
use dotenvy::dotenv;
use std::sync::OnceLock;
use teloxide::Bot;
use tokio::runtime::Handle;

use crate::bot::start_bot;
use crate::config::PLUGIN_ID;

#[export_root_module]
fn get_library() -> PluginRef {
    Plugin {
        get_info,
        init_chat: Some(init_chat),
        handle_chat_respond: Some(handle_chat_respond),
        start_service: None,
    }
    .leak_into_prefix()
}

pub static BOT: OnceLock<Bot> = OnceLock::new();
pub static RUNTIME: OnceLock<Handle> = OnceLock::new();

extern "C" fn init_chat(f: ChatCallback) -> RString {
    dotenv().ok();
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = RUNTIME.set(rt.handle().clone());
        rt.block_on(async {
            start_bot(f).await;
        });
    });
    return PLUGIN_ID.into();
}

extern "C" fn handle_chat_respond(session_id: RString, message: RString) {
    if let (Some(bot), Some(rt)) = (BOT.get(), RUNTIME.get()) {
        let chat_id_str = session_id.to_string();
        let text = message.to_string();
        let bot = bot.clone();

        rt.spawn(async move {
            use teloxide::prelude::*;
            if let Ok(chat_id) = chat_id_str.parse::<i64>() {
                #[allow(deprecated)]
                let _ = bot
                    .send_message(teloxide::types::ChatId(chat_id), text)
                    .parse_mode(teloxide::types::ParseMode::Markdown)
                    .await;
            }
        });
    }
}

extern "C" fn get_info() -> PluginInfo {
    PluginInfo {
        name: "Crusty Plugin Telegram".into(),
        author: "smtdfc".into(),
        version: "0".into(),
        description: "Enables Telegram Bot API integration for Crusty agents. Supports real-time streaming responses, command handling, and session-based conversation persistence.".into(),
    }
}

mod bot;
mod config;
