use rig_core::tool::ToolError;
use rig_derive::rig_tool;

#[rig_tool(description = "Get basic runtime information about the current Crusty process")]
pub async fn get_runtime_info() -> Result<String, ToolError> {
    let current_dir = std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(format!(
        "crate=crusty version={} os={} arch={} pid={} cwd={}",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::process::id(),
        current_dir
    ))
}