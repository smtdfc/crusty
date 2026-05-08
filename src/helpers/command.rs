#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use tracing::error;

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

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn spawn_process(program: &str, args: Vec<&str>) -> bool {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(target_os = "windows")]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let status = command.status();
    match status {
        Err(e) => {
            error!("Error: {}", e);
            false
        }
        _ => true,
    }
}
