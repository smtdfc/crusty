use reqwest::blocking::Client;
use rig_core::tool::ToolError;
use rig_derive::rig_tool;
use serde::Deserialize;
use tracing::info;

// 1. Định nghĩa Struct khớp với cái JSON "khủng bố" kia
#[derive(Deserialize)]
struct WttrResponse {
    current_condition: Vec<CurrentCondition>,
    nearest_area: Vec<NearestArea>,
}

#[derive(Deserialize)]
struct CurrentCondition {
    temp_C: String,
    feelsLikeC: String,
    humidity: String,
    weatherDesc: Vec<ValueWrapper>,
}

#[derive(Deserialize)]
struct NearestArea {
    areaName: Vec<ValueWrapper>,
}

#[derive(Deserialize)]
struct ValueWrapper {
    value: String,
}

#[rig_tool(
    description = "Get current weather for a specific location using wttr.in",
    required(location)
)]
pub fn get_weather(location: String) -> Result<String, ToolError> {
    let client = Client::new();
    let url = format!("https://wttr.in/{}?format=j1", location);
    info!("Tool call");
    let resp = client
        .get(url)
        .send()
        .map_err(|e| ToolError::ToolCallError(format!("Network error: {e}").into()))?;

    let data: WttrResponse = resp
        .json()
        .map_err(|e| ToolError::ToolCallError(format!("JSON parse error: {e}").into()))?;

    let condition = data
        .current_condition
        .get(0)
        .ok_or_else(|| ToolError::ToolCallError("No weather data found".into()))?;

    let area = data
        .nearest_area
        .get(0)
        .map(|a| {
            a.areaName
                .get(0)
                .map(|v| v.value.as_str())
                .unwrap_or("Unknown")
        })
        .unwrap_or("Unknown");

    let desc = condition
        .weatherDesc
        .get(0)
        .map(|v| v.value.as_str())
        .unwrap_or("No description");

    let out = format!(
        "Địa điểm: {}\nNhiệt độ: {}°C (Cảm giác như: {}°C)\nĐộ ẩm: {}%\nTrạng thái: {}",
        area, condition.temp_C, condition.feelsLikeC, condition.humidity, desc
    );

    Ok(out)
}
