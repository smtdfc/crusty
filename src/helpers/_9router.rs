use std::{
    net::{SocketAddr, TcpStream},
    thread,
    time::{Duration, Instant},
};

use crate::helpers::{
    command::{NPX_CMD, spawn_process},
    npm::{check_npm_package, install_npm_package},
};
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize, Debug)]
struct Model {
    id: String,
    object: String,
    owned_by: String,
}

#[derive(Deserialize, Debug)]
struct ModelsResponse {
    data: Vec<Model>,
}

pub fn is_9router_install() -> bool {
    check_npm_package("9router")
}

pub fn ensure_9router_install() {
    let is_install: bool = check_npm_package("9router");
    if !is_install {
        install_npm_package("9router", true);
    }
}

pub fn is_9router_ready(port: i64) -> bool {
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

    match TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn start_9router(port: i64) -> bool {
    spawn_process(
        NPX_CMD,
        vec!["9router", "--no-browser", "--port", &port.to_string()],
    )
}

pub fn ensure_9router_run(port: i64) -> bool {
    if is_9router_ready(port) {
        return true;
    }

    let r = start_9router(port);
    if r {
        let timeout = Duration::from_secs(15);
        let start_time = Instant::now();

        while start_time.elapsed() < timeout {
            if is_9router_ready(port) {
                return true;
            }
            thread::sleep(Duration::from_millis(500));
        }
        error!("Error when start 9router service: {}", "time out ");
    }

    return false;
}

pub async fn fetch_all_models(
    port: i64,
    api_key: &str,
) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("http://localhost:{}/v1/models", port);

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    if response.status().is_success() {
        let models: ModelsResponse = response.json().await?;
        return Ok(models.data);
    } else {
        println!("Error: {}", response.status());
    }

    Ok(vec![])
}
