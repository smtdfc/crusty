use crate::{
    agent::{agent::create_chat_agent, message::ChatMessage},
    config::config::AppConfig,
    helpers::{
        _9router::{ensure_9router_run, is_9router_install},
        tui::{print_banner, show_loading},
    },
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

    if !is_9router_install() {
        print!(
            "{} ",
            style("9Router is not install. Please run crusty setup to setup 9router.")
                .red()
                .bold()
        );

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
        Ok(()) => {}

        Err(s) => {
            print!("{} ", style(s).red().bold());
        }
    };

    let theme = ColorfulTheme::default();
    let term = Term::stdout();

    let Some(model_name) = config.current_model else {
        print!(
            "{} ",
            style("No model select. Please select a model to start chat.")
                .red()
                .bold()
        );
        return;
    };

    let mut agent = create_chat_agent(proxy_config.port, config.api_key, model_name);
    term.clear_screen().unwrap();

    print_banner();
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
