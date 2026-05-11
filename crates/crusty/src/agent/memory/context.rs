use crate::agent::memory::message::ChatRow;
use chrono::Utc;
use rig::message::Message;
use sqlx::{AnyPool, query, query_as};

pub async fn get_context(
    pool: &AnyPool,
    session_id: &str,
    limit: u32,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let rows = query_as::<sqlx::Any, ChatRow>(
        "SELECT * FROM messages WHERE session_id = ? ORDER BY created_at DESC LIMIT ?",
    )
    .bind(session_id)
    .bind(limit as i32)
    .fetch_all(pool)
    .await?;

    let messages = rows
        .into_iter()
        .rev()
        .map(|row| {
            if row.role == "user" {
                Message::user(row.content)
            } else {
                Message::assistant(row.content)
            }
        })
        .collect();

    Ok(messages)
}

pub async fn save_message(
    pool: &AnyPool,
    session_id: &str,
    role: &str,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now().timestamp();

    query("INSERT INTO messages (session_id, role, content, created_at) VALUES (?, ?, ?, ?)")
        .bind(session_id)
        .bind(role)
        .bind(content)
        .bind(now)
        .execute(pool)
        .await?;

    Ok(())
}
