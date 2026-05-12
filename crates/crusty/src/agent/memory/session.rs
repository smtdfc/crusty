use rig::message::Message;
use sqlx::Database;
use tracing::trace;
use uuid::Uuid;

use crate::{
    agent::memory::{context::save_message, store::MemoryStore},
    exceptions::crusty::CrustyError,
};

pub struct Session<'a> {
    pub session_id: String,
    pub store: &'a MemoryStore,
    pub history: Vec<Message>,
}

impl<'a> Session<'a> {
    pub fn new(id: String, store: &'a MemoryStore) -> Self {
        Self {
            session_id: id,
            history: vec![],
            store,
        }
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
        self.history.push(msg);
        Ok(())
    }
}

pub trait MemoryDatabase: Database {}
impl<T: Database> MemoryDatabase for T {}

pub async fn create_session(store: &MemoryStore) -> Result<Session<'_>, CrustyError> {
    let session_id = Uuid::new_v4().to_string();
    let session = Session::new(session_id.clone(), &store);
    trace!("Session {} created", session_id);

    Ok(session)
}
