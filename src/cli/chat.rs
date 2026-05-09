use crate::{
    agent::{agent::create_chat_agent, message::ChatMessage},
    cli::utils::get_active_proxy,
    helpers::tui::{print_banner, print_error, show_loading},
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
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy("start") else {
        return;
    };

    let is_proxy_online = proxy.is_running();
    let theme = ColorfulTheme::default();
    let term = Term::stdout();

    let Some(model_name) = proxy_config.current_model.clone() else {
        print_error("No model select. Please select a model to start chat.");
        return;
    };
    print_banner(
        &model_name,
        &current_proxy,
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
