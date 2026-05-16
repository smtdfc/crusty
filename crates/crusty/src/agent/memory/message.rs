use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ChatRow {
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}
