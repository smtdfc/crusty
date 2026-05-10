use abi_stable::{
    StableAbi,
    std_types::{RStr, RString, RVec},
};

#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct PluginInfo {
    pub name: RString,
    pub author: RString,
    pub version: RString,
    pub description: RString,
}
