use abi_stable::export_root_module;
use abi_stable::prefix_type::PrefixTypeTrait;
use abi_stable::std_types::RString;
use crusty_plugin::plugin::START_SERVICE;
use crusty_plugin::{
    bridge::{ControlCallback, Plugin, PluginRef},
    plugin::PluginInfo,
};

#[export_root_module]
fn get_library() -> PluginRef {
    Plugin {
        get_info,
        start_service,
    }
    .leak_into_prefix()
}

extern "C" fn get_info() -> PluginInfo {
    PluginInfo {
        name: "crusty_plugin_example".into(),
        author: "crusty_plugin_example".into(),
        version: "crusty_plugin_example".into(),
        description: "crusty_plugin_example".into(),
        capabilities: vec![START_SERVICE].into(),
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
