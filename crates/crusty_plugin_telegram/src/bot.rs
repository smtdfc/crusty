use std::env;

use crate::config::PLUGIN_ID;
use crusty_plugin::bridge::ChatCallback;
use teloxide::{Bot, types::Message};

pub async fn start_bot(f: ChatCallback) {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| "".to_string());
    let bot = Bot::new(token);
    let _ = crate::BOT.set(bot.clone());

    teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
        let chat_id = msg.chat.id.0.to_string();
        let user_text = msg.text().unwrap_or("");

        if !user_text.is_empty() {
            (f.ask)(PLUGIN_ID.into(), chat_id.into(), user_text.into());
        }

        Ok(())
    })
    .await;
}
