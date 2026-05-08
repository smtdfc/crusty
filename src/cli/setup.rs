use std::fs;

use console::style;
use dialoguer::Input;

use crate::{
    config::config::AppConfig,
    helpers::_9router::{ensure_9router_install, is_9router_install},
};

pub fn handle_setup() {
    if !is_9router_install() {
        print!("{} ", style("Installing 9router ...").green().bold());
        ensure_9router_install();
        return;
    }

    let path = AppConfig::get_config_path();

    let port: String = Input::new()
        .with_prompt("Enter 9router port")
        .interact_text()
        .unwrap();

    let config_data = serde_json::json!({
        "api_key": "",
        "proxy":{
            "port": port.parse::<i64>().expect("Unable to create config file!")
        }
    });

    fs::write(path, config_data.to_string()).expect("Unable to write file!");

    println!("✅ Saved! Crusty Agent is now available.");
}
