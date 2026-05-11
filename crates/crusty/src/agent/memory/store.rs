use sqlx::AnyPool;
use tracing::trace;

use crate::{config::store::StoreConfig, exceptions::crusty::CrustyError};

pub struct MemoryStore {
    pub pool: AnyPool,
}

impl MemoryStore {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

pub async fn get_store(store_config: &StoreConfig) -> Result<MemoryStore, CrustyError> {
    if store_config.store_type == "sqlite" {
        let pool = AnyPool::connect_lazy(&store_config.uri)?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS messages (
        id TEXT PRIMARY KEY,
        session_id TEXT NOT NULL,
        content TEXT NOT NULL,
        role TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )",
        )
        .execute(&pool)
        .await?;

        trace!(
            "Store {} at {} connected ",
            store_config.store_type, store_config.uri
        );
        return Ok(MemoryStore::new(pool));
    }

    Err(CrustyError::AgentMemoryError(format!(
        "Unsupported database {} ",
        store_config.store_type
    )))
}
