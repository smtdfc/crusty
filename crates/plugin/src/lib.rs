use abi_stable::{
    library::{RootModule, RootModuleStatics},
    sabi_types::VersionStrings,
};

use crate::bridge::PluginRef;

impl RootModule for PluginRef {
    const BASE_NAME: &'static str = "plugin";
    const NAME: &'static str = "plugin";
    const VERSION_STRINGS: VersionStrings = abi_stable::package_version_strings!();

    fn root_module_statics() -> &'static RootModuleStatics<Self> {
        static STATICS: RootModuleStatics<PluginRef> =
            unsafe { RootModuleStatics::__private_new() };
        &STATICS
    }
}

pub mod bridge;
pub mod plugin;
