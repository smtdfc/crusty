use crate::agent::memory::session::Session;
use crate::agent::message::ChatMessage;
use crate::agent::message::TextMessage;
use crate::agent::prompt::SYSTEM_PROMPT;
use crate::agent::tools::calculator::CalculateExpression;
use crate::agent::tools::datetime::GetCurrentDatetime;
use crate::agent::tools::runtime::GetRuntimeInfo;
use crate::agent::tools::weather::GetWeather;
use crate::ai_proxy::ai_proxy::AIProxy;
use crate::config::provider::ProviderConfig;
use crate::exceptions::crusty::CrustyError;
use futures_util::stream::StreamExt;
use rig_core::agent::Agent;
use rig_core::agent::MultiTurnStreamItem;
use rig_core::client::CompletionClient;
use rig_core::completion::CompletionModel;

use rig_core::message::Message;
use rig_core::providers::openai;
use rig_core::streaming::StreamedAssistantContent;
use rig_core::streaming::StreamingChat;
pub struct ChatAgent<T: CompletionModel> {
    agent: Agent<T>,
}

impl<T: CompletionModel + 'static> ChatAgent<T> {
    pub fn new(agent: Agent<T>) -> Self {
        Self { agent: agent }
    }
}

pub type OnMessageCallback = Box<dyn FnMut(ChatMessage) + Send + Sync>;

pub fn create_message_callback(
    f: impl FnMut(ChatMessage) + Send + Sync + 'static,
) -> OnMessageCallback {
    Box::new(f)
}

#[async_trait::async_trait]
pub trait AnyAgent: Send + Sync {
    async fn chat(
        &mut self,
        prompt: &str,
        session: &mut Session,
        mut on_message: OnMessageCallback,
    ) -> Result<(), CrustyError>;
}

#[async_trait::async_trait]
impl<T: CompletionModel + Sync + Send + 'static> AnyAgent for ChatAgent<T> {
    async fn chat(
        &mut self,
        prompt: &str,
        session: &mut Session,
        mut on_message: OnMessageCallback,
    ) -> Result<(), CrustyError> {
        session
            .add_message("user", Message::user(prompt.to_string()))
            .await?;

        let history_lock = session.history.lock().await;
        let history_data = history_lock.clone();
        drop(history_lock);

        let mut stream = self
            .agent
            .stream_chat(prompt, history_data)
            .multi_turn(5)
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

                    StreamedAssistantContent::ToolCall {
                        tool_call,
                        internal_call_id: _,
                    } => {
                        println!("Agent calling tool: {}", tool_call.function.name);
                    }

                    StreamedAssistantContent::ToolCallDelta {
                        id,
                        internal_call_id: _,
                        content: _,
                    } => {
                        println!("Agent calling tool: {}", id);
                    }
                    _ => {}
                },

                MultiTurnStreamItem::StreamUserItem(_) => {}

                MultiTurnStreamItem::FinalResponse(_) => {}

                _ => {}
            }
        }
        session
            .add_message("assistant", Message::assistant(full_response))
            .await?;
        Ok(())
    }
}

fn build_agent(url: &str, api_key: &str, model: &str) -> Box<dyn AnyAgent> {
    let client = openai::Client::builder()
        .api_key(api_key)
        // .http_client(http_client)
        .base_url(url)
        .build();

    let agent = client
        .expect("Cannot create agent")
        .agent(model)
        .preamble(SYSTEM_PROMPT)
        .tool(GetWeather)
        .tool(GetCurrentDatetime)
        .tool(CalculateExpression)
        .tool(GetRuntimeInfo)
        .default_max_turns(5)
        .build();

    Box::new(ChatAgent::new(agent))
}

/// Create a chat agent from a AI proxy configuration
pub fn create_chat_agent_from_proxy(proxy: Box<dyn AIProxy>, model: &str) -> Box<dyn AnyAgent> {
    build_agent(&proxy.get_url(), &proxy.get_api_key(), model)
}

/// Create a chat agent from a provider configuration
/// This supports any OpenAI-compatible API provider
pub fn create_chat_agent_from_provider(
    provider: &ProviderConfig,
    model: &str,
) -> Result<Box<dyn AnyAgent>, CrustyError> {
    if !provider.is_valid() {
        return Err(CrustyError::AgentError(
            "Provider configuration is invalid (missing base_url or api_key)".into(),
        ));
    }

    let normalized_base_url = ProviderConfig::normalize_base_url(&provider.base_url);
    Ok(build_agent(&normalized_base_url, &provider.api_key, model))
}
