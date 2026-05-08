use std::{
    fmt::format,
    net::{SocketAddr, TcpStream},
    thread,
    time::{Duration, Instant},
};

use crate::{
    config::config::AppConfig,
    helpers::{
        npm::{check_npm_package, install_npm_package},
        process::{
            NPX_CMD, get_pids_by_port, save_pid, spawn_process, stop_process, stop_process_by_port,
        },
    },
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

pub fn start_9router(port: i64) -> Result<(), String> {
    spawn_process(
        "9router",
        NPX_CMD,
        vec![
            "9router",
            "--tray",
            "--no-browser",
            "--port",
            &port.to_string(),
        ],
    )?;

    // Try to detect the actual process listening on the port (spawned by npx)
    let timeout = Duration::from_secs(10);
    let start_time = Instant::now();

    while start_time.elapsed() < timeout {
        match get_pids_by_port(port) {
            Ok(pids) if !pids.is_empty() => {
                if let Err(e) = save_pid("9router", pids[0]) {
                    error!("Failed to save detected pid: {}", e);
                }
                return Ok(());
            }
            _ => thread::sleep(Duration::from_millis(200)),
        }
    }

    Err(format!("Failed to start 9router on port {}", port))
}

pub fn stop_9router() -> Result<(), String> {
    let cfg = AppConfig::load().map_err(|e| format!("{}", e))?;
    if let Some(proxy) = cfg.proxy {
        stop_process_by_port(proxy.port).map_err(|e| format!("{}", e))?;
        return Ok(());
    }

    Err(format!("Failed to stop 9router"))
}

pub fn ensure_9router_run(port: i64) -> Result<(), String> {
    if is_9router_ready(port) {
        return Ok(());
    }

    start_9router(port)?;
    let timeout = Duration::from_secs(15);
    let start_time = Instant::now();

    while start_time.elapsed() < timeout {
        if is_9router_ready(port) {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(500));
    }

    error!("Error when start 9router service: {}", "time out ");
    Err(format!("Failed to start 9router"))
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
