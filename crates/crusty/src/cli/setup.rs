use std::collections::HashMap;

use dialoguer::{Confirm, Input, theme::ColorfulTheme};

use crate::{
    ai_proxy::{_9router::_9RouterAIProxy, ai_proxy::AIProxy},
    config::{ai_proxy::AIProxyConfig, config::AppConfig},
    helpers::tui::{print_error, print_info, print_success, show_menu},
};

fn setup_9router() -> AIProxyConfig {
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

    let proxy = _9RouterAIProxy { port };
    if !proxy.is_install() {
        print_info("Installing 9router ...");
        let _ = proxy.install();
    }

    print_info(&format!(
        "Please follow the steps below to complete the setup:
            if your proxy run locally, please run crusty proxy start to start the proxy.
            Open: http://{}:{}/dashboard/combos in your browser (Log in if prompted)
            Add a combo box named crusty_combo so that crusty can function.
        ",
        host, port
    ));

    AIProxyConfig {
        platform: format!("9router"),
        is_local,
        host,
        port,
        api_key: None,
        current_model: Some(format!("crusty_combo")),
    }
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

    let mut config = AppConfig {
        current_proxy: None,
        ai_proxies: HashMap::new(),
        plugins: vec![],
    };

    if select == 0 {
        let c = setup_9router();
        config.ai_proxies.insert(name.clone(), c);
        config.current_proxy = Some(name);
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
