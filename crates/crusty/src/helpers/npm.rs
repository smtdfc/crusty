use crate::{exceptions::crusty::CrustyError, helpers::process::NPM_CMD};
use std::process::Command;
use tracing::error;

pub fn check_npm_package(package_name: &str) -> Result<bool, CrustyError> {
    let output = Command::new(NPM_CMD)
        .args(["list", package_name, "--depth=0", "--json"])
        .output();

    match output {
        Ok(out) => Ok(out.status.success()),
        Err(e) => {
            error!("Error when check package {} with NPM: {}", package_name, e);
            Err(CrustyError::PackageError(format!("{}", e)))
        }
    }
}

pub fn install_npm_package(package_name: &str, global: bool) -> Result<bool, CrustyError> {
    let mut args = vec!["install", package_name];

    if global {
        args.push("-g");
    }

    let output = Command::new(NPM_CMD)
        .args(["install", package_name])
        .status();

    match output {
        Ok(status) => Ok(status.success()),
        Err(e) => {
            error!(
                "Error when install package {} with NPM: {}",
                package_name, e
            );

            Err(CrustyError::PackageError(format!("{}", e)))
        }
    }
}
