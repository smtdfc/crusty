use abi_stable::{StableAbi, std_types::RString};

use crate::plugin::PluginInfo;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PluginRef)))]
pub struct Plugin {
    pub get_info: extern "C" fn() -> PluginInfo,
    pub start_service: Option<extern "C" fn(callback: ControlCallback)>,
    pub init_chat: Option<extern "C" fn(ChatCallback) -> RString>,
    pub handle_chat_respond: Option<extern "C" fn(session_id: RString, message: RString)>,
}

#[repr(C)]
#[derive(StableAbi)]
pub struct ChatCallback {
    pub ask: extern "C" fn(RString, RString, RString), // args: plugin id, session id, msg
}

#[repr(C)]
#[derive(StableAbi)]
pub struct ControlCallback {
    pub on_message: extern "C" fn(RString, RString),
}
