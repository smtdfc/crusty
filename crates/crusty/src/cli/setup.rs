use std::collections::HashMap;

use dialoguer::{Confirm, Input, theme::ColorfulTheme};
use tracing::{error, info, trace};

use crate::{
    ai_proxy::{_9router::_9RouterAIProxy, ai_proxy::AIProxy},
    config::{ai_proxy::AIProxyConfig, config::AppConfig, store::StoreConfig},
    exceptions::crusty::CrustyError,
    helpers::tui::{print_error, print_info, print_success, show_menu},
};

fn setup_9router() -> Result<AIProxyConfig, CrustyError> {
    let theme = ColorfulTheme::default();
    let port: u64;
    let mut host = format!("localhost");
    let is_local: bool = Confirm::with_theme(&theme)
        .with_prompt("Use 9router locally?")
        .default(true)
        .interact()
        .unwrap();

    if !is_local {
        host = Input::with_theme(&theme)
            .with_prompt("What is 9router host ?")
            .interact_text()
            .unwrap();
    }

    port = Input::with_theme(&theme)
        .with_prompt("What is 9router port ?")
        .interact_text()
        .unwrap();

    let proxy = _9RouterAIProxy {
        host,
        port,
        is_local,
    };
    match proxy.is_install() {
        Ok(false) => {
            print_info("Installing 9router ...");
            proxy.install()?;
        }

        Err(e) => {
            error!(?e, "Check install failed");
            return Err(e.into());
        }

        Ok(_) => {
            info!("Proxy already installed, skipping...");
        }
    }

    print_info(&format!(
        "Please follow the steps below to complete the setup:
            if your proxy run locally, please run crusty proxy start to start the proxy.
            Open: http://{}:{}/dashboard/combos in your browser (Log in if prompted)
            Add a combo box named crusty_combo so that crusty can function.
        ",
        &proxy.host, &proxy.port
    ));

    Ok(AIProxyConfig {
        platform: format!("9router"),
        is_local,
        host: proxy.host,
        port,
        api_key: None,
        current_model: Some(format!("crusty_combo")),
    })
}

pub fn handle_setup() {
    let theme = ColorfulTheme::default();
    let Some(select) = show_menu(vec!["9router"], "Select proxy platform") else {
        return;
    };

    let name: String = Input::with_theme(&theme)
        .with_prompt("What is proxy name ?")
        .interact_text()
        .unwrap();

    let mut db_path = AppConfig::get_data_dir();
    db_path.push("store.db");

    let mut config = AppConfig {
        current_proxy: None,
        ai_proxies: HashMap::new(),
        plugins: vec![],
        store: Some(StoreConfig {
            store_type: "sqlite".into(),
            uri: format!(
                "sqlite:{}?mode=rwc",
                db_path.to_str().unwrap().replace("\\", "/")
            ),
        }),
    };

    if select == 0 {
        match setup_9router() {
            Ok(c) => {
                config.ai_proxies.insert(name.clone(), c);
                config.current_proxy = Some(name);
            }

            Err(e) => {
                error!(error = ?e, "Failed to setup 9router status");
                print_error(&format!("Failed to setup 9router. Please try again"));
                return;
            }
        };
    }

    match config.save() {
        Ok(()) => {
            print_success("Saved! Crusty Agent is now available.");
        }

        Err(e) => {
            print_error(&format!("Error {}", e));
        }
    }
}
