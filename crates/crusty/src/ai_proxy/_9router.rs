use std::{
    net::{SocketAddr, TcpStream},
    thread,
    time::{Duration, Instant},
};

use tracing::error;

use crate::{
    ai_proxy::ai_proxy::AIProxy,
    helpers::{
        npm::{check_npm_package, install_npm_package},
        process::{NPX_CMD, get_pids_by_port, save_pid, spawn_process, stop_process_by_port},
    },
};

pub struct _9RouterAIProxy {
    pub port: u64,
}

impl AIProxy for _9RouterAIProxy {
    fn is_install(&self) -> bool {
        check_npm_package("9router")
    }

    fn is_running(&self) -> bool {
        let addr: SocketAddr = format!("127.0.0.1:{}", self.port).parse().unwrap();

        match TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn get_url(&self) -> String {
        format!("localhost:{}", self.port)
    }

    fn start(&self) -> Result<(), String> {
        spawn_process(
            "9router",
            NPX_CMD,
            vec![
                "9router",
                "--tray",
                "--no-browser",
                "--port",
                &self.port.to_string(),
            ],
        )?;

        // Try to detect the actual process listening on the port (spawned by npx)
        let timeout = Duration::from_secs(10);
        let start_time = Instant::now();

        while start_time.elapsed() < timeout {
            match get_pids_by_port(self.port) {
                Ok(pids) if !pids.is_empty() => {
                    if let Err(e) = save_pid("9router", pids[0]) {
                        error!("Failed to save detected pid: {}", e);
                    }
                    return Ok(());
                }
                _ => thread::sleep(Duration::from_millis(200)),
            }
        }

        Err(format!("Failed to start 9router on port {}", self.port))
    }

    fn stop(&self) -> Result<(), String> {
        stop_process_by_port(self.port).map_err(|e| format!("{}", e))?;
        Ok(())
    }

    fn install(&self) -> Result<(), String> {
        install_npm_package("9router", true);
        Ok(())
    }
}
