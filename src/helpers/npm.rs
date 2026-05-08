use crate::helpers::command::NPM_CMD;
use std::process::Command;
use tracing::error;

pub fn check_npm_package(package_name: &str) -> bool {
    let output = Command::new(NPM_CMD)
        .args(["list", package_name, "--depth=0", "--json"])
        .output();

    match output {
        Ok(out) => out.status.success(),
        Err(e) => {
            error!("Error when check package {} with NPM: {}", package_name, e);
            false
        }
    }
}

pub fn install_npm_package(package_name: &str, global: bool) -> bool {
    let mut args = vec!["install", package_name];

    if global {
        args.push("-g");
    }

    let output = Command::new(NPM_CMD)
        .args(["install", package_name])
        .status();

    match output {
        Ok(status) => status.success(),
        Err(e) => {
            error!(
                "Error when install package {} with NPM: {}",
                package_name, e
            );

            false
        }
    }
}
