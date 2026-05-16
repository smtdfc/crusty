use crate::{
    agent::{
        agent::{create_chat_agent, create_message_callback},
        memory::{session::create_session, store::SharedMemoryStore},
        message::ChatMessage,
    },
    cli::utils::{get_active_proxy_and_check, get_agent_params},
    helpers::tui::{print_error, show_loading},
};
use console::{Term, style};
use dialoguer::{Input, theme::ColorfulTheme};
use std::io::Write;
use tracing::error;

pub async fn handle_chat_start(memory_store: &SharedMemoryStore) {
    let Some((_current_proxy, proxy_config, _proxy)) = get_active_proxy_and_check("start", false)
    else {
        return;
    };

    let Some((model_name, api_key)) = get_agent_params(&proxy_config) else {
        return;
    };

    let theme = ColorfulTheme::default();
    let term = Term::stdout();

    let mut session = match create_session(memory_store.clone()).await {
        Ok(s) => s,
        Err(e) => {
            error!(error = ?e, "Failed to create session");
            print_error(&format!("Cannot init chat session now. Cause: {}", e));
            return;
        }
    };

    let mut agent = create_chat_agent(proxy_config.port, &api_key, &model_name);
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

                let callback = create_message_callback(move |m| {
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
                });

                agent
                    .chat(&prompt, &mut session, callback)
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
