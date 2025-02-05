mod neural;
mod bot;
mod web;
mod network;
mod autonomous;

use crate::neural::cortex::Cortex;
use crate::bot::telegram::TelegramBot;
use crate::web::WebServer;
use crate::network::p2p::P2PNetwork;
use crate::autonomous;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация
    let cortex = Cortex::load().await?;
    let tg_bot = TelegramBot::new(cortex.clone());
    let web = WebServer::new(cortex.clone());
    let p2p = P2PNetwork::boot(cortex).await?;

    // Запуск подсистем
    tokio::try_join!(
        tg_bot.run(),
        web.start(),
        p2p.listen(),
        autonomous::start_cycle()
    )?;

    Ok(())
}
