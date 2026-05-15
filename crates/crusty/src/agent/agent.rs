use crate::agent::memory::session::Session;
use crate::agent::message::ChatMessage;
use crate::agent::message::TextMessage;
use crate::agent::prompt::SYSTEM_PROMPT;
use crate::exceptions::crusty::CrustyError;
use futures_util::stream::StreamExt;
use rig::agent::Agent;
use rig::agent::MultiTurnStreamItem;
use rig::client::CompletionClient;
use rig::message::Message;
use rig::providers::openai;
use rig::streaming::StreamedAssistantContent;
use rig::streaming::StreamingChat;

use tracing::info;
use tracing::trace;
pub struct ChatAgent<T: rig::completion::CompletionModel> {
    agent: Agent<T>,
}

impl<T: rig::completion::CompletionModel + 'static> ChatAgent<T> {
    pub fn new(agent: Agent<T>) -> Self {
        Self { agent: agent }
    }

    pub async fn chat<F>(
        &mut self,
        prompt: &str,
        session: &mut Session<'_>,
        mut on_message: F,
    ) -> Result<(), CrustyError>
    where
        F: FnMut(ChatMessage) + Send + Sync + 'static,
    {
        session
            .add_message("user", Message::user(prompt.to_string()))
            .await?;
        let mut stream = self
            .agent
            .stream_chat(prompt, session.history.clone())
            .await;
        let mut full_response = String::new();

        while let Some(chunk) = stream.next().await {
            let stream_obj = chunk.map_err(|e| {
                CrustyError::AgentError(format!("Failed to send request. Cause: {}", e))
            })?;

            match stream_obj {
                MultiTurnStreamItem::StreamAssistantItem(content) => match content {
                    StreamedAssistantContent::Text(t) => {
                        full_response.push_str(&t.text);
                        on_message(ChatMessage::TextMessage(TextMessage {
                            role: "model".into(),
                            content: t.text,
                        }));
                    }
                    _ => {}
                },

                _ => {}
            }
        }
        session
            .add_message("assistant", Message::assistant(full_response))
            .await?;
        Ok(())
    }
}

pub fn create_chat_agent(
    port: u64,
    api_key: &str,
    model: &str,
) -> ChatAgent<impl rig::completion::CompletionModel + use<>> {
    // let http_client = reqwest::Client::new();
    let client = openai::Client::builder()
        .api_key(api_key)
        // .http_client(http_client)
        .base_url(format!("http://localhost:{}/v1", port))
        .build();

    let agent = client
        .expect("Cannot create agent")
        .agent(model)
        .preamble(SYSTEM_PROMPT)
        .build();

    info!(
        "Agent initialization successful. AI Proxy port: {}; Model: {}",
        port, model
    );
    ChatAgent::new(agent)
}
