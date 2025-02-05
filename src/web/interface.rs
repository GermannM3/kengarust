use axum::{routing::get, Router, response::Html};

pub struct WebServer {
    cortex: Arc<Cortex>,
}

impl WebServer {
    pub fn new(cortex: Arc<Cortex>) -> Self {
        Self { cortex }
    }

    pub async fn start(&self) {
        let app = Router::new()
            .route("/", get(Self::chat_interface))
            .route("/status", get(Self::system_status))
            .route("/api/ask", post(Self::process_query))
            .route("/ws", get(Self::websocket_handler));

        axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    async fn chat_interface() -> Html<&'static str> {
        Html(include_str!("../../templates/chat.html"))
    }

    async fn system_status(&self) -> Json<Status> {
        Json(self.cortex.get_full_status().await)
    }
}
