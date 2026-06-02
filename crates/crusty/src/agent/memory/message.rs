#[derive(sqlx::FromRow)]
pub struct ChatRow {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<String>,
    pub tool_results: Option<String>,
    pub tool_call_id: Option<String>,
    pub name: Option<String>,
    pub created_at: i64,
}
