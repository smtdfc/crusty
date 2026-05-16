use crate::{
    agent::{
        agent::{create_chat_agent, create_chat_agent_from_provider, create_message_callback},
        memory::{session::create_session, store::SharedMemoryStore},
        message::ChatMessage,
    },
    cli::utils::{get_active_provider_and_check, get_active_proxy_and_check, get_agent_params},
    config::config::{AppConfig, RunMode},
    config::provider::ProviderConfig,
    helpers::tui::{print_error, show_loading},
};
use console::{Term, style};
use dialoguer::{Input, theme::ColorfulTheme};
use std::io::Write;
use tracing::error;

pub async fn handle_chat_start(memory_store: &SharedMemoryStore) {
    // Get the current running mode
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(?e, "Failed to load config");
            print_error(&format!("Failed to load config: {}", e));
            return;
        }
    };

    let current_mode = match config.get_mode() {
        Ok(mode) => mode,
        Err(e) => {
            error!(?e, "No mode selected");
            print_error(&format!("{}", e));
            return;
        }
    };

    // Load appropriate config based on selected mode
    let (provider_name, provider_config, model_name, agent_result) = match current_mode {
        RunMode::Provider => {
            // Must use provider in provider mode
            match get_active_provider_and_check() {
                Some((provider_name, provider_config)) => {
                    let Some(raw_model) = provider_config.default_model.clone() else {
                        error!("Provider mode selected but provider has no default model");
                        print_error(
                            "Provider mode is active but no default model is configured. Please re-run 'crusty provider add' and set a valid model.",
                        );
                        return;
                    };

                    let agent_result =
                        create_chat_agent_from_provider(&provider_config, &raw_model);
                    (
                        Some(provider_name),
                        Some(provider_config),
                        raw_model,
                        agent_result,
                    )
                }
                None => {
                    error!("Provider mode selected but no active provider configured");
                    print_error(
                        "Provider mode is active but no provider is configured. Please run 'crusty provider add' to add a provider.",
                    );
                    return;
                }
            }
        }
        RunMode::Proxy => {
            // Must use proxy in proxy mode
            match get_active_proxy_and_check("chat", false) {
                Some((_current_proxy, proxy_config, _proxy)) => {
                    match get_agent_params(&proxy_config) {
                        Some((model_name, api_key)) => {
                            let model_name_clone = model_name.clone();
                            (
                                None,
                                None,
                                model_name,
                                Ok(create_chat_agent(
                                    proxy_config.port,
                                    &api_key,
                                    &model_name_clone,
                                )),
                            )
                        }
                        None => {
                            error!("Failed to get agent parameters");
                            return;
                        }
                    }
                }
                None => {
                    error!("Proxy mode selected but no active proxy configured");
                    print_error(
                        "Proxy mode is active but no proxy is configured. Please run 'crusty setup' to configure a proxy.",
                    );
                    return;
                }
            }
        }
    };

    let mut agent = match agent_result {
        Ok(agent) => agent,
        Err(e) => {
            error!(error = ?e, "Failed to create agent");
            print_error(&format!("Cannot initialize agent. Cause: {}", e));
            return;
        }
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
                        let error_message = format!("{}", e);
                        error!(error = ?e, "Failed to load chat");
                        print_error(&format!("Cannot load message. Cause: {}", error_message));

                        if current_mode == RunMode::Provider && error_message.contains("404") {
                            let provider_label = provider_name
                                .as_ref()
                                .map_or("<unknown>", |name| name.as_str());
                            let provider_base_url = provider_config
                                .as_ref()
                                .map_or("<unknown>", |cfg| cfg.base_url.as_str());
                            print_error(&format!(
                                "Provider '{}' returned 404. Check Base URL and model. Current base_url='{}', model='{}'. For OpenAI-compatible APIs, base_url usually ends with '/v1'.",
                                provider_label, provider_base_url, model_name
                            ));
                        }
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
