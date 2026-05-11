use std::sync::Arc;

use rig::message::Message;
use sqlx::Database;
use uuid::Uuid;

use crate::{
    agent::memory::store::{MemoryStore, get_store},
    config::store::StoreConfig,
};

pub struct Session {
    pub session_id: String,
    pub store: Arc<MemoryStore>,
    pub history: Vec<Message>,
}

impl Session {
    pub fn new(id: String, store: Arc<MemoryStore>) -> Self {
        Self {
            session_id: id,
            history: vec![],
            store,
        }
    }
}

pub trait MemoryDatabase: Database {}
impl<T: Database> MemoryDatabase for T {}

pub fn create_session(store_config: &StoreConfig) -> Option<Session> {
    let session_id = Uuid::new_v4().to_string();
    let Ok(inner_store) = get_store(&store_config) else {
        return None;
    };

    let session = Session::new(session_id, Arc::new(inner_store));
    return Some(session);
}
