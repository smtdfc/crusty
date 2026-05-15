use abi_stable::export_root_module;
use abi_stable::prefix_type::PrefixTypeTrait;
use abi_stable::std_types::RString;
use crusty_plugin::bridge::ChatCallback;
use crusty_plugin::{
    bridge::{Plugin, PluginRef},
    plugin::PluginInfo,
};
use dotenvy::dotenv;

use crate::bot::start_bot;
use crate::config::PLUGIN_ID;

#[export_root_module]
fn get_library() -> PluginRef {
    Plugin {
        get_info,
        init_chat: Some(init_chat),
        handle_chat_respond: None,
        start_service: None,
    }
    .leak_into_prefix()
}

extern "C" fn init_chat(f: ChatCallback) -> RString {
    dotenv().ok();
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            start_bot(f).await;
        });
    });
    return PLUGIN_ID.into();
}

extern "C" fn get_info() -> PluginInfo {
    PluginInfo {
        name: "crusty plugin telegram".into(),
        author: "smtdfc".into(),
        version: "0".into(),
        description: "crusty plugin telegram".into(),
    }
}

mod bot;
mod config;
