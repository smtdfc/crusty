use crate::{agent::memory::message::ChatRow, exceptions::crusty::CrustyError};
use chrono::Utc;
use rig_core::message::Message;
use sqlx::{AnyPool, query, query_as};
use tracing::info;

pub async fn get_context(
    pool: &AnyPool,
    session_id: &str,
    limit: u32,
) -> Result<Vec<Message>, CrustyError> {
    let rows = match query_as::<sqlx::Any, ChatRow>(
        r#"
    SELECT 
        session_id, 
        role, 
        content, 
        CAST(created_at AS TEXT) as created_at 
    FROM messages 
    WHERE session_id = ? 
    ORDER BY created_at DESC 
    LIMIT ?
    "#,
    )
    .bind(session_id)
    .bind(limit as i32)
    .fetch_all(pool)
    .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("SQLX QUERY ERROR: {:?}", e);
            return Err(CrustyError::AgentMemoryError(format!(
                "Failed to query. Cause: {}",
                e
            )));
        }
    };

    let messages: Vec<Message> = rows
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

    info!(
        "Query message for session: {} with limit {}. Successful with {} message(s)",
        session_id,
        limit,
        messages.len()
    );

    Ok(messages)
}

pub async fn save_message(
    pool: &AnyPool,
    session_id: &str,
    role: &str,
    content: &str,
) -> Result<(), CrustyError> {
    let now = Utc::now().timestamp();
    let id = uuid::Uuid::new_v4().to_string();

    query(
        "INSERT INTO messages (id, session_id, role, content, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(id)
    .bind(session_id)
    .bind(role)
    .bind(content)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| CrustyError::AgentMemoryError(format!("Failed to query. Cause: {}", e)))?;

    info!(
        "Query save message for session: {} with role {}. Successful",
        session_id, role
    );

    Ok(())
}
