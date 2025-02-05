use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            // Здесь можно добавить обработку сообщений через веб-сокеты
            let response = format!("Echo: {}", text);
            if socket.send(Message::Text(response)).await.is_err() {
                break;
            }
        }
    }
}

pub fn router() -> Router {
    Router::new().route("/ws", get(ws_handler))
}

