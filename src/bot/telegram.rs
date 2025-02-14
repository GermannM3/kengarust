use teloxide::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::nlp::{recognize_intent, analyze_sentiment};

/// Запускает Telegram-бота с использованием teloxide.
pub async fn run_telegram_bot() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Запуск Telegram-бота...");

    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN не задан");
    let bot = Bot::new(bot_token);

    teloxide::repl(bot, |message: Message, bot: Bot| async move {
        if let Some(text) = message.text() {
            let intent = recognize_intent(text);
            let sentiment = analyze_sentiment(text);
            let response = match intent.as_str() {
                "Погода" => "Сегодня солнечно и тепло!".to_string(),
                "Новости" => "В мире ничего нового.".to_string(),
                _ => format!("Тональность: {}. Спасибо за общение!", sentiment),
            };

            if let Err(err) = bot.send_message(message.chat.id, response).await {
                log::error!("Ошибка отправки: {:?}", err);
            }
        }
        Ok(())
    })
    .await;
}

/// Экспортируем функцию для внешнего использования.
pub async fn run_bot() {
    run_telegram_bot().await;
}

// Заглушка для Telegram‑бота
pub struct TelegramBot;

impl TelegramBot {
    pub fn new() -> Self {
        TelegramBot
    }
    
    pub fn start(&self) {
        // Реализуйте запуск Telegram‑бота здесь
    }
}

