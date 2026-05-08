use crate::{
    agent::{agent::create_chat_agent, message::ChatMessage},
    config::{self, config::AppConfig},
    helpers::{
        _9router::{ensure_9router_install, ensure_9router_run, is_9router_install},
        tui::{print_banner, show_loading},
    },
};
use clap::Subcommand;
use console::{Term, style};
use dialoguer::{Input, theme::ColorfulTheme};
use std::io::Write;
use sysinfo::Cpu;

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
            style("9Router not install. Please run crusty setup to setup 9router.")
                .red()
                .bold()
        );

        return;
    }

    ensure_9router_run(config.proxy.port);

    let theme = ColorfulTheme::default();
    let term = Term::stdout();
    let mut agent = create_chat_agent(config.proxy.port, config.api_key, config.current_model);
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
