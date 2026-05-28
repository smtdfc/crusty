use std::collections::HashMap;

use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use tracing::{error, info};

use crate::{
    ai_proxy::ai_proxy::get_proxy,
    config::{
        ai_proxy::AIProxyConfig,
        config::{AppConfig, RunMode},
        provider::ProviderConfig,
        store::StoreConfig,
    },
    exceptions::crusty::CrustyError,
    helpers::tui::{print_error, print_info, print_success, show_menu},
};

fn setup_9router() -> Result<AIProxyConfig, CrustyError> {
    let theme = ColorfulTheme::default();
    let mut host = String::from("localhost");
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

    let port = if is_local {
        Input::with_theme(&theme)
            .with_prompt("What is 9router port ?")
            .interact_text()
            .unwrap()
    } else {
        8000
    };

    let config = AIProxyConfig {
        platform: String::from("9router"),
        is_local,
        host,
        port,
        api_key: None,
        current_model: Some(String::from("crusty_combo")),
    };

    let Some(proxy) = get_proxy("9router", &config) else {
        return Err(CrustyError::ConfigError(format!(
            "Unsupported proxy platform: {}",
            "9router"
        )));
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
            If your proxy runs locally, please run crusty proxy start to start the proxy.
            Open: {}{} in your browser (Log in if prompted)
            Add a combo box named crusty_combo so that crusty can function.
        ",
        proxy.get_dashboard_url(),
        "/combos"
    ));

    Ok(config)
}

fn setup_omniroute() -> Result<AIProxyConfig, CrustyError> {
    let theme = ColorfulTheme::default();
    let mut host = String::from("127.0.0.1");
    let is_local: bool = Confirm::with_theme(&theme)
        .with_prompt("Use OmniRoute locally?")
        .default(true)
        .interact()
        .unwrap();

    if !is_local {
        host = Input::with_theme(&theme)
            .with_prompt("What is OmniRoute host ?")
            .interact_text()
            .unwrap();
    }

    let port = if is_local {
        Input::with_theme(&theme)
            .with_prompt("What is OmniRoute port ?")
            .interact_text()
            .unwrap()
    } else {
        8000
    };

    let config = AIProxyConfig {
        platform: String::from("omniroute"),
        is_local,
        host,
        port,
        api_key: None,
        current_model: Some(String::from("crusty_combo")),
    };

    let Some(proxy) = get_proxy("omniroute", &config) else {
        return Err(CrustyError::ConfigError(String::from(
            "Unsupported proxy platform: OmniRoute",
        )));
    };

    match proxy.is_install() {
        Ok(false) => {
            print_info("Installing OmniRoute ...");
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
            If your proxy runs locally, please run crusty proxy start to start the proxy.
            Open: {} in your browser (Log in if prompted)
            Add a combo box named crusty_combo so that crusty can function.
        ",
        proxy.get_dashboard_url()
    ));

    Ok(config)
}

fn setup_provider() -> Result<(String, ProviderConfig), CrustyError> {
    let theme = ColorfulTheme::default();

    let name: String = Input::with_theme(&theme)
        .with_prompt("Provider name")
        .interact_text()
        .unwrap_or_default();

    if name.is_empty() {
        return Err(CrustyError::ConfigError(
            "Provider name cannot be empty".into(),
        ));
    }

    let providers_types = vec!["openai", "anthropic", "custom"];
    let provider_type_idx = Select::with_theme(&theme)
        .with_prompt("Select provider type")
        .items(&providers_types)
        .interact()
        .unwrap_or(0);

    let base_url: String = Input::with_theme(&theme)
        .with_prompt("Base URL (e.g., https://api.openai.com/v1)")
        .interact_text()
        .unwrap_or_default();

    if base_url.is_empty() {
        return Err(CrustyError::ConfigError("Base URL cannot be empty".into()));
    }

    let api_key: String = Input::with_theme(&theme)
        .with_prompt("API Key")
        .interact_text()
        .unwrap_or_default();

    if api_key.is_empty() {
        return Err(CrustyError::ConfigError("API Key cannot be empty".into()));
    }

    let default_model: String = Input::with_theme(&theme)
        .with_prompt("Default model (required)")
        .interact_text()
        .unwrap_or_default();

    if default_model.is_empty() {
        return Err(CrustyError::ConfigError(
            "Default model cannot be empty".into(),
        ));
    }

    let provider = ProviderConfig::new(
        providers_types[provider_type_idx],
        &base_url,
        &api_key,
        Some(default_model),
    );

    Ok((name, provider))
}

pub fn handle_setup() {
    let theme = ColorfulTheme::default();
    let mode_items = vec!["Proxy", "Provider"];
    let mode_select = Select::with_theme(&theme)
        .with_prompt("Select mode")
        .items(&mode_items)
        .default(0)
        .interact()
        .unwrap_or(0);

    let mut db_path = AppConfig::get_data_dir();
    db_path.push("store.db");

    let mut config = AppConfig {
        mode: None,
        current_proxy: None,
        ai_proxies: HashMap::new(),
        current_provider: None,
        providers: HashMap::new(),
        plugins: vec![],
        store: Some(StoreConfig {
            store_type: "sqlite".into(),
            uri: format!(
                "sqlite:{}?mode=rwc",
                db_path.to_str().unwrap().replace("\\", "/")
            ),
        }),
    };

    let mode = if mode_select == 1 {
        match setup_provider() {
            Ok((provider_name, provider_config)) => {
                config
                    .providers
                    .insert(provider_name.clone(), provider_config);
                config.current_provider = Some(provider_name);
                RunMode::Provider
            }
            Err(e) => {
                error!(error = ?e, "Failed to setup provider");
                print_error(&format!("Failed to setup provider: {}", e));
                return;
            }
        }
    } else {
        let Some(select) = show_menu(vec!["9router", "omniroute"], "Select proxy platform") else {
            return;
        };

        let name: String = Input::with_theme(&theme)
            .with_prompt("What is proxy name ?")
            .interact_text()
            .unwrap();

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
        } else if select == 1 {
            match setup_omniroute() {
                Ok(c) => {
                    config.ai_proxies.insert(name.clone(), c);
                    config.current_proxy = Some(name);
                }

                Err(e) => {
                    error!(error = ?e, "Failed to setup omniroute status");
                    print_error(&format!("Failed to setup omniroute. Please try again"));
                    return;
                }
            };
        }

        RunMode::Proxy
    };

    if let Err(e) = config.set_mode(mode) {
        error!(error = ?e, "Failed to set mode");
        print_error(&format!("Failed to set mode: {}", e));
        return;
    }

    match config.save() {
        Ok(()) => {
            print_success("Saved! Crusty Agent is now available.");
            print_info(&format!("Current mode: {}", mode));
        }

        Err(e) => {
            print_error(&format!("Error {}", e));
        }
    }
}
