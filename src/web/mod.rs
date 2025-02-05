pub mod api;
pub mod interface;
pub mod ws;
pub use interface::WebServer; // если WebServer объявлен публично в interface.rs


/// Запуск веб-интерфейса (с объединением API и интерфейса).
pub async fn run_web_interface() {
    // Объединяем маршруты из интерфейса и API
    let app = axum::Router::new()
        .merge(api::router())
        .merge(interface::router())
        .merge(ws::router());
    let addr = "0.0.0.0:8080".parse().unwrap();
    println!("Веб-интерфейс доступен на http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

