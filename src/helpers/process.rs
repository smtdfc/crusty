#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use sysinfo::{Pid, ProcessesToUpdate, System};
use tracing::error;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::config::config::AppConfig;

pub static NPM_CMD: &str = if cfg!(target_os = "windows") {
    "npm.cmd"
} else {
    "npm"
};

pub static NPX_CMD: &str = if cfg!(target_os = "windows") {
    "npx.cmd"
} else {
    "npx"
};

// 0x08000000 -> CREATE_NO_WINDOW
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(target_os = "windows")]
pub fn spawn_process(key: &str, program: &str, args: Vec<&str>) -> Result<u32, String> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

    #[cfg(target_os = "windows")]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    match command.spawn() {
        Ok(child) => {
            let pid = child.id();
            save_pid(key, pid);
            Ok(pid)
        }
        Err(e) => {
            error!("Error: {}", e);
            Err(format!("Failed to spawn process"))
        }
    }
}

fn get_pid_file_path(key: &str) -> PathBuf {
    AppConfig::get_config_dir().join(format!("process_{}.pid", key))
}

pub fn save_pid(key: &str, pid: u32) -> Result<(), Box<dyn Error>> {
    let path = get_pid_file_path(key);
    fs::write(path, pid.to_string())?;
    Ok(())
}

pub fn get_queued_pid(key: &str) -> Option<u32> {
    let path = get_pid_file_path(key);
    let content = fs::read_to_string(path).ok()?;
    content.trim().parse::<u32>().ok()
}

pub fn clear_pid(key: &str) -> std::io::Result<()> {
    let path = get_pid_file_path(key);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn get_validated_process(key: &str, expected_name: &str) -> Option<u32> {
    let pid = get_queued_pid(key)?;

    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::Some(&[Pid::from(pid as usize)]), true);

    if let Some(process) = sys.process(Pid::from(pid as usize)) {
        let name = process.name().to_string_lossy();
        let exe_path = process
            .exe()
            .map(|p| p.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        if name.contains(&expected_name.to_lowercase())
            || exe_path.contains(&expected_name.to_lowercase())
        {
            return Some(pid);
        }
    }

    None
}

pub fn stop_process(key: &str) -> Result<(), Box<dyn Error>> {
    // Try to read queued PID and kill the process directly. Some spawned
    // processes (eg. via `npx`) may exit and leave their child (tray) process
    // running — so also scan all processes for ones related to `9router` and
    // kill them by matching name/exe/cmdline as a fallback.

    if let Some(pid) = get_queued_pid(key) {
        let mut sys = System::new_all();
        sys.refresh_all();

        if let Some(p) = sys.process(Pid::from(pid as usize)) {
            let _ = p.kill();
        }
    }

    clear_pid(key)?;
    Ok(())
}

pub fn stop_process_by_port(port: i64) -> Result<(), Box<dyn Error>> {
    let mut pids: std::collections::HashSet<u32> = std::collections::HashSet::new();

    if cfg!(target_os = "windows") {
        // -a: all, -n: numerical, -o: owner (PID)
        let output = Command::new("netstat").args(&["-ano"]).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let local_address = parts[1];
                if local_address.ends_with(&format!(":{}", port)) {
                    if let Ok(pid) = parts.last().unwrap().parse::<u32>() {
                        if pid > 0 {
                            pids.insert(pid);
                        }
                    }
                }
            }
        }
    } else {
        if let Ok(output) = Command::new("lsof")
            .args(&["-i", &format!(":{}", port), "-t"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Ok(pid) = line.trim().parse::<u32>() {
                    pids.insert(pid);
                }
            }
        }
    }

    if pids.is_empty() {
        return Ok(());
    }

    for pid in pids {
        println!("Killing PID: {}", pid);
        if cfg!(target_os = "windows") {
            let _ = Command::new("taskkill")
                .args(&["/F", "/T", "/PID", &pid.to_string()])
                .output();
        } else {
            let mut sys = System::new_all();
            sys.refresh_processes(
                ProcessesToUpdate::Some(&[Pid::from(pid as usize)]),
                true, // remove_dead_processes
            );

            if let Some(proc) = sys.process(Pid::from(pid as usize)) {
                let _ = proc.kill();
            }
            if let Some(proc) = sys.process(Pid::from(pid as usize)) {
                proc.kill();
            }
        }
    }

    Ok(())
}

pub fn get_pids_by_port(port: i64) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut pids: Vec<u32> = vec![];

    if cfg!(target_os = "windows") {
        let output = Command::new("netstat").args(&["-ano"]).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let local = parts[1];
                let pid_token = parts.last().unwrap();
                if local.ends_with(&format!(":{}", port)) {
                    if let Ok(pid) = pid_token.parse::<u32>() {
                        pids.push(pid);
                    }
                }
            }
        }
    } else {
        if let Ok(output) = Command::new("lsof")
            .args(&["-i", &format!(":{}", port), "-t"])
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if let Ok(pid) = line.trim().parse::<u32>() {
                        pids.push(pid);
                    }
                }
            }
        }
    }

    Ok(pids)
}
