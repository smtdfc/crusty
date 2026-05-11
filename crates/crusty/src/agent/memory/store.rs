use sqlx::AnyPool;

use crate::config::store::StoreConfig;

pub struct MemoryStore {
    pub pool: AnyPool,
}

impl MemoryStore {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

pub fn get_store(store_config: &StoreConfig) -> Result<MemoryStore, String> {
    if store_config.store_type == "sqlite" {
        let pool = AnyPool::connect_lazy(&store_config.uri).map_err(|e| format!("{}", e))?;
        return Ok(MemoryStore::new(pool));
    }

    Err(format!(""))
}
