use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrustyError {
    #[error("Database Error: {0}")]
    SqliteError(#[from] sqlx::Error),

    #[error("AI Proxy Error: {0}")]
    AIProxyError(String),

    #[error("Agent Memory Error: {0}")]
    AgentMemoryError(String),

    #[error("Agent Error: {0}")]
    AgentError(String),

    #[error("Invalid Data Error: {0}")]
    InvalidDataError(String),

    #[error("Package Error: {0}")]
    PackageError(String),

    #[error("Process Error: {0}")]
    ProcessError(String),

    #[error("Plugin Error: {0}")]
    PluginError(String),

    #[error("Config Error: {0}")]
    ConfigError(String),

    #[error("Tool Error: {0}")]
    ToolError(String),
}
