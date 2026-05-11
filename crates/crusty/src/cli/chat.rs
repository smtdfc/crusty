use crate::{
    agent::{agent::create_chat_agent, memory::session::create_session, message::ChatMessage},
    cli::utils::get_active_proxy,
    config::config::AppConfig,
    helpers::tui::{print_error, show_loading},
};
use console::{Term, style};
use dialoguer::{Input, theme::ColorfulTheme};
use std::io::Write;

pub async fn handle_chat_start(config: &AppConfig) {
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy(&config, "start") else {
        return;
    };

    let Some(ref store_config) = config.store else {
        return;
    };
    let is_proxy_online = proxy.is_running();
    let theme = ColorfulTheme::default();
    let term = Term::stdout();

    let Some(model_name) = proxy_config.current_model.clone() else {
        print_error("No model select. Please select a model to start chat.");
        return;
    };

    if !is_proxy_online {
        print_error(
            format!(
                "Proxy {} (platform: {}) is offline. Please run proxy before.",
                current_proxy, proxy_config.platform
            )
            .as_str(),
        );
        return;
    }

    let api_key = proxy_config.api_key.as_deref().unwrap_or("").to_string();
    let mut session = create_session(&store_config).expect("Cannot create session");
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
                    .chat(&prompt, &mut session, move |m| {
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
