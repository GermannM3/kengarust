mod bot;
mod web;
mod neural;
mod autonomous;
mod network;
mod utils;
mod social;

use crate::neural::cortex::Cortex;
use crate::bot::telegram::TelegramBot;
use crate::web::interface::WebServer;
use crate::network::p2p::P2PNetwork;
use crate::autonomous::learning::start_cycle; // Убедитесь, что эта функция существует

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация подсистем с передачей ссылки на Cortex (пример)
    let cortex = Cortex::load().await?; // Предположим, что Cortex::load() существует и возвращает Result<Cortex, _>
    let tg_bot = TelegramBot::new(cortex.clone());
    let web = WebServer::new(cortex.clone());
    let p2p = P2PNetwork::boot(cortex).await?;

    // Запуск подсистем параллельно
    tokio::try_join!(
        tg_bot.run(),
        web.start(),
        p2p.listen(),
        start_cycle()
    )?;

    Ok(())
}

