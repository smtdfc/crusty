use crate::{
    agent::{agent::create_chat_agent, memory::session::create_session, message::ChatMessage},
    cli::utils::get_active_proxy,
    config::config::AppConfig,
    helpers::tui::{print_error, show_loading},
};
use console::{Term, style};
use dialoguer::{Input, theme::ColorfulTheme};
use std::io::Write;
use tracing::error;

pub async fn handle_chat_start(config: &AppConfig) {
    let Some((current_proxy, proxy_config, proxy)) = get_active_proxy(&config, "start") else {
        return;
    };

    let Some(ref store_config) = config.store else {
        print_error("No store config. Please config storage first.");
        return;
    };

    let theme = ColorfulTheme::default();
    let term = Term::stdout();

    let Some(model_name) = proxy_config.current_model.clone() else {
        print_error("No model select. Please select a model to start chat.");
        return;
    };

    match proxy.is_running() {
        Ok(false) => {
            print_error(&format!(
                "Proxy {} (platform: {}) is offline. Please run proxy before.",
                current_proxy, proxy_config.platform
            ));
            return;
        }

        Err(e) => {
            error!(error = ?e, "Failed to check proxy status");
            print_error(&format!("Failed to check proxy. Please try again"));
            return;
        }

        Ok(true) => {}
    }

    let api_key = match proxy_config.api_key.as_deref() {
        None => String::from(""),
        Some(v) => v.to_string(),
    };

    let mut session = match create_session(&store_config).await {
        Ok(s) => s,
        Err(e) => {
            error!(error = ?e, "Failed to create session");
            print_error(&format!("Cannot init chat session now. Cause: {}", e));
            return;
        }
    };

    let mut agent = create_chat_agent(proxy_config.port, api_key, model_name);
    term.clear_screen().unwrap_or_else(|e| {
        print_error(&format!("Failed to init console. Cause: {}", e));
    });

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
                        error!(error = ?e, "Failed to load chat");
                        print_error(&format!("Cannot load message. Cause: {}", e));
                        std::io::stdout().flush().unwrap();
                    });

                println!("\n");
            }
            Err(e) => {
                error!(error = ?e, "Failed to send message");
                print_error(&format!("Failed to get input from std::io. Cause: {}", e));
                return;
            }
        }
    }
}
