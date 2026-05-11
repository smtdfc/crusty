use abi_stable::export_root_module;
use abi_stable::prefix_type::PrefixTypeTrait;
use abi_stable::std_types::RString;
use crusty_plugin::bridge::ChatCallback;
use crusty_plugin::{
    bridge::{ControlCallback, Plugin, PluginRef},
    plugin::PluginInfo,
};
// teloxide usage moved into `bot` module; hide unused prelude here

use crate::bot::start_bot;

#[export_root_module]
fn get_library() -> PluginRef {
    Plugin {
        get_info,
        start_service: Some(start_service),
        init_chat: Some(init_chat),
        handle_chat_respond: None,
    }
    .leak_into_prefix()
}

extern "C" fn init_chat(f: ChatCallback) -> RString {
    start_bot();
    return "1".into();
}

extern "C" fn get_info() -> PluginInfo {
    PluginInfo {
        name: "crusty_plugin_telegram".into(),
        author: "smtdfc".into(),
        version: "0".into(),
        description: "crusty_plugin_telegram".into(),
    }
}

extern "C" fn start_service(callback: ControlCallback) {
    std::thread::spawn(move || {
        println!("Plugin Telegram đang khởi động...");

        std::thread::sleep(std::time::Duration::from_secs(3));

        let chat_id = RString::from("user_123");
        let content = RString::from("Chào sếp, em là Telegram Plugin đây!");

        (callback.on_message)(chat_id, content);
    });
}

mod bot;
