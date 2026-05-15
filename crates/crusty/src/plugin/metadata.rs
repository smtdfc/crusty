use std::{fs::File, path::Path};

use serde::Deserialize;
use std::io::BufReader;
use tracing::{info, trace};

use crate::exceptions::crusty::CrustyError;

#[derive(Deserialize, Debug)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub file: String,
}

#[derive(Deserialize, Debug)]
pub struct PluginMetadata {
    pub name: String,
    pub id: String,
    pub version: Option<String>,
    pub platforms: Vec<PlatformInfo>,
    pub features: Option<Vec<String>>,
}

impl PluginMetadata {
    pub fn find_compatible_binary(&self) -> Option<&String> {
        let current_os = std::env::consts::OS;
        let current_arch = std::env::consts::ARCH;

        self.platforms
            .iter()
            .find(|p| p.os == current_os && p.arch == current_arch)
            .map(|p| &p.file)
    }
}

pub fn read_metadata<P: AsRef<Path>>(file_path: P) -> Result<PluginMetadata, CrustyError> {
    let file = File::open(file_path.as_ref()).map_err(|e| {
        CrustyError::PluginError(format!(
            "Failed to read metadata file {}. Cause: {}",
            file_path.as_ref().to_str().unwrap(),
            e
        ))
    })?;

    let reader = BufReader::new(file);
    let metadata: PluginMetadata = serde_json::from_reader(reader).map_err(|e| {
        CrustyError::PluginError(format!(
            "Failed to read metadata file {}. Cause: {}",
            file_path.as_ref().to_str().unwrap(),
            e
        ))
    })?;

    info!(
        "[Plugin] Read metadata file: {}",
        file_path.as_ref().display()
    );

    Ok(metadata)
}
