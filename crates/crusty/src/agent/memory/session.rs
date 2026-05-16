use std::sync::Arc;

use rig::message::Message;
use sqlx::Database;
use tracing::info;
use uuid::Uuid;

use crate::{
    agent::memory::{
        context::{get_context, save_message},
        store::SharedMemoryStore,
    },
    exceptions::crusty::CrustyError,
    helpers::types::{ArcMutex, new_arc_mutex},
};

pub type History = ArcMutex<Vec<Message>>;
pub struct Session {
    pub session_id: String,
    pub store: SharedMemoryStore,
    pub history: History,
}

impl Session {
    pub fn new(id: String, store: &SharedMemoryStore) -> Self {
        Self {
            session_id: id,
            history: new_arc_mutex(vec![]),
            store: Arc::clone(&store),
        }
    }

    pub async fn load(id: String, store: &SharedMemoryStore) -> Result<Self, CrustyError> {
        let history = get_context(&store.pool, &id, 100).await?;
        Ok(Self {
            session_id: id,
            history: new_arc_mutex(history),
            store: Arc::clone(&store),
        })
    }

    pub async fn add_message(&mut self, role: &str, msg: Message) -> Result<(), CrustyError> {
        let content = match &msg {
            Message::User { content } => content
                .iter()
                .map(|c| match c {
                    rig::message::UserContent::Text(t) => t.text.as_str(),
                    _ => "",
                })
                .collect::<Vec<_>>()
                .join("\n"),

            Message::Assistant { content, .. } => content
                .iter()
                .filter_map(|c| {
                    if let rig::completion::AssistantContent::Text(t) = c {
                        Some(t.text.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join("\n"),

            _ => String::new(),
        };

        save_message(&self.store.pool, self.session_id.as_str(), role, &content).await?;
        let mut history_lock = self.history.lock().await;
        history_lock.push(msg);

        Ok(())
    }
}

pub trait MemoryDatabase: Database {}
impl<T: Database> MemoryDatabase for T {}

pub async fn create_session(store: SharedMemoryStore) -> Result<Session, CrustyError> {
    let session_id = Uuid::new_v4().to_string();
    let session = Session::new(session_id.clone(), &store);
    info!("Session {} created", session_id);

    Ok(session)
}
