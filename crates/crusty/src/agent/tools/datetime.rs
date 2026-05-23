use chrono::Local;
use rig_core::tool::ToolError;
use rig_derive::rig_tool;

#[rig_tool(description = "Get the current local date and time")]
pub async fn get_current_datetime() -> Result<String, ToolError> {
    let now = Local::now();
    Ok(format!("{}", now.to_rfc3339()))
}