use crate::agent::message::ChatMessage;
use crate::agent::message::TextMessage;
use crate::agent::prompt::SYSTEM_PROMPT;
use futures_util::stream::StreamExt;
use rig::agent::Agent;
use rig::agent::MultiTurnStreamItem;
use rig::client::CompletionClient;
use rig::message::Message;
use rig::providers::openai;
use rig::streaming::StreamedAssistantContent;
use rig::streaming::StreamingChat;

use tracing::error;
pub struct ChatAgent<T: rig::completion::CompletionModel> {
    agent: Agent<T>,
    history: Vec<Message>,
}

impl<T: rig::completion::CompletionModel + 'static> ChatAgent<T> {
    pub fn new(agent: Agent<T>) -> Self {
        Self {
            agent: agent,
            history: vec![],
        }
    }

    pub async fn chat<F>(&mut self, prompt: &str, mut on_message: F) -> Result<(), String>
    where
        F: FnMut(ChatMessage) + Send + Sync + 'static,
    {
        self.history.push(Message::user(prompt.to_string()));
        let mut stream = self.agent.stream_chat(prompt, self.history.clone()).await;
        let mut full_response = String::new();

        while let Some(chunk) = stream.next().await {
            let stream_obj = chunk.map_err(|e| {
                error!("Error: {:?}", e);
                format!("Failed to send request. Check log for details")
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
        self.history.push(Message::assistant(full_response));
        Ok(())
    }
}

pub fn create_chat_agent(
    port: i64,
    api_key: String,
    model: String,
) -> ChatAgent<impl rig::completion::CompletionModel> {
    let client = openai::Client::builder()
        .api_key(api_key)
        .base_url(format!("http://localhost:{}/v1", port))
        .build();

    let agent = client
        .expect("Cannot create agent")
        .agent(model)
        .preamble(SYSTEM_PROMPT)
        .build();

    ChatAgent::new(agent)
}
