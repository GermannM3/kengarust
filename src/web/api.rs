use axum::{routing::post, Json, Router};

pub async fn start_api_server(cortex: Arc<Cortex>) {
    let app = Router::new()
        .route("/ask", post(handle_ask))
        .with_state(cortex);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_ask(Json(query): Json<String>, cortex: State<Arc<Cortex>>) -> Json<String> {
    Json(cortex.process(&query).await)
}
