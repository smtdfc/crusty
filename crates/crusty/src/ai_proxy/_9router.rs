use std::{
    net::{SocketAddr, TcpStream},
    thread,
    time::{Duration, Instant},
};

use tracing::{info, trace};

use crate::{
    ai_proxy::ai_proxy::AIProxy,
    exceptions::crusty::CrustyError,
    helpers::{
        npm::{check_npm_package, install_npm_package},
        process::{NPX_CMD, get_pids_by_port, save_pid, spawn_process, stop_process_by_port},
    },
};

pub struct _9RouterAIProxy {
    pub host: String,
    pub port: u64,
    pub is_local: bool,
}

impl AIProxy for _9RouterAIProxy {
    fn is_install(&self) -> Result<bool, CrustyError> {
        check_npm_package("9router")
    }

    fn is_running(&self) -> Result<bool, CrustyError> {
        let addr: SocketAddr = format!("127.0.0.1:{}", self.port).parse().unwrap();

        match TcpStream::connect_timeout(&addr, Duration::from_millis(500)) {
            Ok(_) => Ok(true),
            Err(_e) => Ok(false),
        }
    }

    fn get_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    fn start(&self) -> Result<(), CrustyError> {
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
                    save_pid("9router", pids[0])?;
                    info!("9router started");
                    return Ok(());
                }
                _ => thread::sleep(Duration::from_millis(200)),
            }
        }

        Err(CrustyError::AIProxyError(format!(
            "Failed to start 9router on port {}",
            self.port
        )))
    }

    fn stop(&self) -> Result<(), CrustyError> {
        stop_process_by_port(self.port)?;
        info!("9router stopped");
        Ok(())
    }

    fn install(&self) -> Result<(), CrustyError> {
        install_npm_package("9router", true)?;
        Ok(())
    }

    fn get_dashboard_url(&self) -> String {
        format!("http://{}:{}/dashboard", self.host, self.port)
    }
}
