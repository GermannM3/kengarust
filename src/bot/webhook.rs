// Данный модуль содержит заглушку для обработки webhook-запросов.
// В реальной системе можно реализовать серверный обработчик webhook.
use axum::{
    extract::Json,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct WebhookPayload {
    message: String,
}

/// Обработчик webhook-запроса.
async fn webhook_handler(Json(payload): Json<WebhookPayload>) -> impl IntoResponse {
    println!("Получен webhook: {}", payload.message);
    "Webhook обработан"
}

/// Функция для запуска сервера webhook.
pub async fn run_webhook_server() {
    let app = Router::new().route("/webhook", post(webhook_handler));
    let addr = "0.0.0.0:8081".parse().unwrap();
    println!("Webhook сервер запущен на {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

