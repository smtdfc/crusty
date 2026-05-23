use rig_core::tool::ToolError;
use rig_derive::rig_tool;

#[rig_tool(
    description = "Evaluate a basic arithmetic expression using +, -, *, /, parentheses, and decimals",
    required(expression)
)]
pub async fn calculate_expression(expression: String) -> Result<String, ToolError> {
    let result = meval::eval_str(&expression)
        .map_err(|e| ToolError::ToolCallError(format!("Calculation error: {e}").into()))?;

    Ok(format!("{} = {}", expression, result))
}