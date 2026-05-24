use crate::{
    cli::{
        mode::{handle_mode_show, handle_mode_switch},
        plugin::{handle_plugin_install, handle_plugin_list, handle_plugin_remove},
        provider::{
            handle_provider_add, handle_provider_list, handle_provider_remove,
            handle_provider_switch,
        },
        proxy::{
            handle_proxy_add, handle_proxy_dashboard, handle_proxy_edit, handle_proxy_list,
            handle_proxy_remove, handle_proxy_start, handle_proxy_stop, handle_proxy_switch,
        },
        setup::handle_setup,
        store::{handle_store_edit, handle_store_reset, handle_store_show},
    },
    helpers::tui::{print_info, show_loading, show_menu},
};

pub fn handle_config() {
    show_loading("Preparing ...");

    loop {
        if !handle_config_menu() {
            break;
        }

        print_info("Returning to configuration menu...");
    }
}

fn handle_config_menu() -> bool {
    let Some(choice) = show_menu(
        vec![
            "Mode settings",
            "Provider settings",
            "Plugin settings",
            "Proxy settings",
            "Store settings",
            "Setup wizard",
            "Back",
        ],
        "Select a configuration area",
    ) else {
        return false;
    };

    match choice {
        0 => handle_mode_settings_menu(),
        1 => handle_provider_settings_menu(),
        2 => handle_plugin_settings_menu(),
        3 => handle_proxy_settings_menu(),
        4 => handle_store_settings_menu(),
        5 => handle_setup(),
        _ => return false,
    }

    true
}

fn handle_mode_settings_menu() {
    let Some(action) = show_menu(vec!["Show mode", "Switch mode", "Back"], "Mode settings") else {
        return;
    };

    match action {
        0 => handle_mode_show(),
        1 => handle_mode_switch(),
        _ => {}
    }
}

fn handle_provider_settings_menu() {
    let Some(action) = show_menu(
        vec![
            "List providers",
            "Add provider",
            "Remove provider",
            "Switch provider",
            "Back",
        ],
        "Provider settings",
    ) else {
        return;
    };

    match action {
        0 => handle_provider_list(),
        1 => handle_provider_add(),
        2 => handle_provider_remove(),
        3 => handle_provider_switch(),
        _ => {}
    }
}

fn handle_proxy_settings_menu() {
    let Some(action) = show_menu(
        vec![
            "List proxies",
            "Add proxy",
            "Edit proxy",
            "Remove proxy",
            "Switch proxy",
            "Start proxy",
            "Stop proxy",
            "Open dashboard",
            "Back",
        ],
        "Proxy settings",
    ) else {
        return;
    };

    match action {
        0 => handle_proxy_list(),
        1 => handle_proxy_add(),
        2 => handle_proxy_edit(),
        3 => handle_proxy_remove(),
        4 => handle_proxy_switch(),
        5 => handle_proxy_start(),
        6 => handle_proxy_stop(),
        7 => handle_proxy_dashboard(),
        _ => {}
    }
}

fn handle_plugin_settings_menu() {
    let Some(action) = show_menu(
        vec!["List plugins", "Install plugin", "Remove plugin", "Back"],
        "Plugin settings",
    ) else {
        return;
    };

    match action {
        0 => handle_plugin_list(),
        1 => handle_plugin_install_prompt(),
        2 => handle_plugin_remove(),
        _ => {}
    }
}

fn handle_plugin_install_prompt() {
    let Some(path) = dialoguer::Input::<String>::new()
        .with_prompt("Plugin path")
        .interact_text()
        .ok()
    else {
        return;
    };

    if path.trim().is_empty() {
        return;
    }

    handle_plugin_install(path.trim());
}

fn handle_store_settings_menu() {
    let Some(action) = show_menu(
        vec!["Show store", "Edit store path", "Reset store", "Back"],
        "Store settings",
    ) else {
        return;
    };

    match action {
        0 => handle_store_show(),
        1 => handle_store_edit(),
        2 => handle_store_reset(),
        _ => {}
    }
}
