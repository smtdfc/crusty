use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StoreConfig {
    pub store_type: String,
    pub uri: String,
}
