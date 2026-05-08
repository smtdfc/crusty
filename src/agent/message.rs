pub struct TextMessage {
    pub role: String,
    pub content: String,
}

pub enum ChatMessage {
    TextMessage(TextMessage),
}
