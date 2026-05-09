use crate::{
    agent::{agent::create_chat_agent, message::ChatMessage},
    ai_proxy::ai_proxy::{AIProxy, get_proxy},
    config::config::AppConfig,
    helpers::tui::{print_banner, show_loading},
};
use clap::Subcommand;
use console::{Term, style};
use dialoguer::{Input, theme::ColorfulTheme};
use std::io::Write;

#[derive(Subcommand)]
pub enum ChatCommands {
    Start {},
}

pub async fn handle_chat_start() {
    show_loading("Preparing ...");
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);

            return;
        }
    };

    let Some(current_proxy) = &config.current_proxy else {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            "No proxy select. Please setup first."
        );
        return;
    };

    let Some(proxy_config) = config.find_proxy_by_id(current_proxy) else {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            "No proxy found. Please setup first."
        );
        return;
    };

    if !proxy_config.is_local {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            format!(
                "Proxy {} (platform {}) is remote proxy from another address cannot start locally",
                current_proxy, proxy_config.platform
            )
        );
    }

    let Some(proxy) = get_proxy(&proxy_config.platform, &proxy_config) else {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            "Failed to init proxy. Please check logs for details."
        );
        return;
    };

    if !proxy.is_install() {
        eprintln!(
            "{} {}",
            style("Error:").red().bold(),
            format!(
                "Platform {} (for {}) not install. Please run crusty setup first.",
                proxy_config.platform, current_proxy
            )
        );
        return;
    };

    let is_proxy_online = proxy.is_running();
    let theme = ColorfulTheme::default();
    let term = Term::stdout();

    let Some(model_name) = proxy_config.current_model.clone() else {
        print!(
            "{} ",
            style("No model select. Please select a model to start chat.")
                .red()
                .bold()
        );
        return;
    };
    print_banner(
        &model_name,
        current_proxy,
        &proxy_config.platform,
        &proxy_config.host,
        proxy_config.port,
        is_proxy_online,
    );

    let api_key = proxy_config.api_key.as_deref().unwrap_or("").to_string();
    let mut agent = create_chat_agent(proxy_config.port, api_key, model_name);
    term.clear_screen().unwrap();

    loop {
        let input: Result<String, _> = Input::with_theme(&theme).with_prompt("You").interact_text();

        match input {
            Ok(prompt) => {
                if prompt.trim().is_empty() {
                    continue;
                }

                let pb = show_loading("Agent: Thinking ...");
                let mut first_chunk = true;

                agent
                    .chat(&prompt, move |m| {
                        if first_chunk {
                            pb.finish_and_clear();
                            print!("{} ", style("Agent:").green().bold());
                            first_chunk = false;
                        }

                        match m {
                            ChatMessage::TextMessage(chunk) => {
                                print!("{}", chunk.content);
                                std::io::stdout().flush().unwrap();
                            }
                        }
                    })
                    .await
                    .unwrap_or_else(|e| {
                        print!("{} ", style("System:").red().bold());
                        print!("{}", e);
                        std::io::stdout().flush().unwrap();
                    });

                println!("\n");
            }
            Err(_) => {
                break;
            }
        }
    }
}
