use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ChatRow {
    pub id: i32,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: i64,
}
