use reqwest::blocking::Client;
use rig_core::tool::ToolError;
use rig_derive::rig_tool;
use serde::Deserialize;
use std::env;

#[rig_tool(
    description = "Perform basic arithmetic operations",
    required(x, y, operation)
)]
fn get_weather(location: String) -> Result<String, ToolError> {
    let api_key = env::var("OPENWEATHER_API_KEY")
        .map_err(|_| ToolError::ToolCallError("OPENWEATHER_API_KEY not set".into()))?;

    let client = Client::builder()
        .build()
        .map_err(|e| ToolError::ToolCallError(format!("http client build error: {e}").into()))?;

    let units = "metric".to_string();
    let url = reqwest::Url::parse_with_params(
        "https://api.openweathermap.org/data/2.5/weather",
        &[("q", &location), ("appid", &api_key), ("units", &units)],
    )
    .map_err(|e| ToolError::ToolCallError(format!("url build error: {e}").into()))?;

    let resp = client
        .get(url)
        .send()
        .map_err(|e| ToolError::ToolCallError(format!("request error: {e}").into()))?
        .error_for_status()
        .map_err(|e| ToolError::ToolCallError(format!("http status error: {e}").into()))?;

    #[derive(Deserialize)]
    struct WeatherResp {
        name: String,
        weather: Vec<Weather>,
        main: Main,
    }

    #[derive(Deserialize)]
    struct Weather {
        description: String,
    }

    #[derive(Deserialize)]
    struct Main {
        temp: f64,
        humidity: Option<u64>,
    }

    let w: WeatherResp = resp
        .json()
        .map_err(|e| ToolError::ToolCallError(format!("json parse error: {e}").into()))?;

    let desc = w
        .weather
        .get(0)
        .map(|w| w.description.clone())
        .unwrap_or_default();
    let out = format!(
        "Location: {}\nTemperature: {} °C\nConditions: {}",
        w.name, w.main.temp, desc
    );
    Ok(out)
}
