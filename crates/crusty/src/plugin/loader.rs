use abi_stable::library::RootModule;
use crusty_plugin::bridge::PluginRef;
use tracing::{info, trace};

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    config::{config::AppConfig, plugin::PluginConfig},
    exceptions::crusty::CrustyError,
    helpers::fs::copy_dir_all,
    plugin::metadata::read_metadata,
};

pub fn load_plugin(path: &Path) -> PluginRef {
    let plugin = PluginRef::load_from_file(path).expect("Cannot load plugin file!");

    plugin
}

pub fn install_plugin(path: &str, config: &mut AppConfig) -> Result<(), CrustyError> {
    let base = PathBuf::from(path);
    let metadata_file = base.join("metadata.json");
    let mut dest_path = PathBuf::from(AppConfig::get_data_dir());

    if !metadata_file.is_file() {
        return Err(CrustyError::PluginError(format!(
            "metadata.json file not found at {:?}",
            base
        )));
    }

    let metadata = read_metadata(&metadata_file)?;
    let id = metadata.id.clone();
    let parts: Vec<&str> = id.split('/').collect();
    for part in parts {
        dest_path.push(part);
    }

    if let Err(e) = std::fs::create_dir_all(&dest_path) {
        return Err(CrustyError::PluginError(format!(
            "Failed to create dir. Cause {:?}",
            e
        )));
    }

    let binary_rel_path = metadata.find_compatible_binary().ok_or_else(|| {
        return CrustyError::PluginError(format!(
            "The '{}' plugin is not supported on this operating system or architecture! ",
            metadata.name
        ));
    })?;

    let binary_full_path = base.join(binary_rel_path);
    if !binary_full_path.is_file() {
        return Err(CrustyError::PluginError(format!(
            "The binary file '{}' is declared but does not exist!",
            binary_rel_path
        )));
    }

    let binary_dest_path = dest_path.join(binary_rel_path);

    if dest_path.exists() {
        let _ = fs::remove_dir_all(&dest_path).map_err(|e| {
            return CrustyError::PluginError(format!("Failed to remove dir. Cause {:?}", e));
        });
    }

    config.plugins.push(PluginConfig {
        name: metadata.name,
        id: metadata.id,
        file: binary_dest_path
            .to_str()
            .expect("Path not is UTF-8 format")
            .to_string(),
        features: metadata.features,
    });

    copy_dir_all(&base, &dest_path).map_err(|e| {
        return CrustyError::PluginError(format!("Failed to copy content. Cause {:?}", e));
    })?;

    info!("[Plugin] Installed plugin {} (id: {})", path, id);
    info!("Installed plugin {}", id);
    Ok(())
}
