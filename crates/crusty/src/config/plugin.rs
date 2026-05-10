use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub id: String,
    pub file: String,
}
