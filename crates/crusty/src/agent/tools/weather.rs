use reqwest::Client;
use rig_core::tool::ToolError;
use rig_derive::rig_tool;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct WttrResponse {
    current_condition: Vec<CurrentCondition>,
    nearest_area: Vec<NearestArea>,
    #[serde(default)]
    request: Vec<RequestInfo>,
    #[serde(default)]
    weather: Vec<WeatherDay>,
}

#[derive(Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,
    #[serde(rename = "FeelsLikeC", alias = "feelsLikeC", default)]
    feels_like_c: Option<String>,
    humidity: String,
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<ValueWrapper>,
}

#[derive(Deserialize)]
struct NearestArea {
    #[serde(rename = "areaName")]
    area_name: Vec<ValueWrapper>,
    #[serde(default)]
    country: Vec<ValueWrapper>,
    #[serde(default)]
    region: Vec<ValueWrapper>,
    #[serde(default)]
    weather_url: Vec<ValueWrapper>,
}

#[derive(Deserialize)]
struct RequestInfo {
    query: String,
    #[serde(rename = "type")]
    request_type: String,
}

#[derive(Deserialize)]
struct WeatherDay {
    date: String,
    #[serde(rename = "avgtempC")]
    avg_temp_c: String,
    #[serde(rename = "maxtempC")]
    max_temp_c: String,
    #[serde(rename = "mintempC")]
    min_temp_c: String,
    #[serde(default)]
    hourly: Vec<HourlyForecast>,
}

#[derive(Deserialize)]
struct HourlyForecast {
    #[serde(rename = "tempC")]
    temp_c: String,
    #[serde(rename = "FeelsLikeC", alias = "feelsLikeC", default)]
    feels_like_c: Option<String>,
    humidity: String,
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<ValueWrapper>,
}

#[derive(Deserialize)]
struct ValueWrapper {
    value: String,
}

#[rig_tool(
    description = "Get current weather for a specific location using wttr.in",
    required(location)
)]
pub async fn get_weather(location: String) -> Result<String, ToolError> {
    let client = Client::new();
    let url = format!("https://wttr.in/{}?format=j1", location);
    info!("Tool call");
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| ToolError::ToolCallError(format!("Network error: {e}").into()))?;

    let data: WttrResponse = resp
        .json()
        .await
        .map_err(|e| ToolError::ToolCallError(format!("JSON parse error: {e}").into()))?;

    let condition = data
        .current_condition
        .get(0)
        .ok_or_else(|| ToolError::ToolCallError("No weather data found".into()))?;

    let area = data
        .nearest_area
        .get(0)
        .map(|a| {
            a.area_name
                .get(0)
                .map(|v| v.value.as_str())
                .unwrap_or("Unknown")
        })
        .unwrap_or("Unknown");

    let desc = condition
        .weather_desc
        .get(0)
        .map(|v| v.value.as_str())
        .unwrap_or("No description");

    let out = format!(
        "Location: {} Temperature: {}°C (Feels like: {}°C) Humidity: {}% State: {}",
        area,
        condition.temp_c,
        condition.feels_like_c.as_deref().unwrap_or("Unknown"),
        condition.humidity,
        desc
    );

    Ok(out)
}
