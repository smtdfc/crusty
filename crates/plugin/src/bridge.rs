use abi_stable::{StableAbi, std_types::RString};

use crate::plugin::PluginInfo;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PluginRef)))]
pub struct Plugin {
    pub get_info: extern "C" fn() -> PluginInfo,
    pub start_service: extern "C" fn(callback: ControlCallback),
}

#[repr(C)]
#[derive(StableAbi)]
pub struct ControlCallback {
    pub on_message: extern "C" fn(RString, RString),
}
