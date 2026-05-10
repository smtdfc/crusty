use abi_stable::library::RootModule;
use crusty_plugin::bridge::PluginRef;

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    config::{config::AppConfig, plugin::PluginConfig},
    helpers::fs::{copy_dir_all, is_it_really_a_file},
    plugin::metadata::read_metadata,
};

pub fn load_plugin(path: &Path) -> PluginRef {
    let plugin = PluginRef::load_from_directory(path.parent().unwrap())
        .expect("Cannot load plugin! Looks like there's a layout or version mismatch");

    plugin
}

pub fn install_plugin(
    path: &str,
    config: &mut AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let base = PathBuf::from(path);
    let metadata_file = base.join("metadata.json");
    let mut dest_path = PathBuf::from(AppConfig::get_data_dir());

    if !metadata_file.is_file() {
        return Err(format!("Error: Metadata.json not found at {:?}", base).into());
    }

    let metadata = read_metadata(&metadata_file)?;
    dest_path.push(&metadata.id);
    let binary_rel_path = metadata.find_compatible_binary().ok_or_else(|| {
        format!(
            "The '{}' plugin is not supported on this operating system or architecture! ",
            metadata.name
        )
    })?;

    let binary_full_path = base.join(binary_rel_path);
    if !binary_full_path.is_file() {
        return Err(format!(
            "Error: The binary file '{}' is declared but does not exist!",
            binary_rel_path
        )
        .into());
    }

    if dest_path.exists() {
        fs::remove_dir_all(&dest_path)?;
    }

    config.plugins.push(PluginConfig {
        name: metadata.name,
        id: metadata.id,
        file: dest_path
            .to_str()
            .expect("Path not is UTF-8 format")
            .to_string(),
    });

    copy_dir_all(&base, &dest_path)?;
    Ok(())
}
