use clap::Subcommand;
use console::style;

use crate::{
    config::config::AppConfig,
    helpers::{
        _9router::{ensure_9router_run, is_9router_install, stop_9router},
        tui::show_loading,
    },
};

#[derive(Subcommand)]
pub enum ProxyCommands {
    Start {},
    Stop {},
    Dashboard {},
}

pub fn handle_proxy_start() {
    show_loading("Preparing ...");
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);

            return;
        }
    };

    if !is_9router_install() {
        return;
    }

    let Some(proxy_config) = config.proxy else {
        print!(
            "{} ",
            style("9Router is not setup. Please run crusty setup to setup 9router.")
                .red()
                .bold()
        );

        return;
    };

    match ensure_9router_run(proxy_config.port) {
        Ok(()) => {
            print!(
                "{} ",
                style(format!("9Router is running in port {}", proxy_config.port))
                    .green()
                    .bold()
            );
        }

        Err(s) => {
            print!("{} ", style(s).red().bold());
        }
    };
}

pub fn handle_proxy_stop() {
    show_loading("Preparing ...");
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);

            return;
        }
    };

    if !is_9router_install() {
        return;
    }

    let Some(_) = config.proxy else {
        print!(
            "{} ",
            style("9Router is not setup. Please run crusty setup to setup 9router.")
                .red()
                .bold()
        );

        return;
    };

    match stop_9router() {
        Ok(()) => {
            print!(
                "{} ",
                style(format!("9Router has been stopped")).green().bold()
            );
        }

        Err(s) => {
            print!("{} ", style(s).red().bold());
        }
    };
}
