use crate::{modules::Modules, routes::point::point};
use axum::{extract::DefaultBodyLimit, routing::post, Router};
use std::sync::Arc;

pub async fn startup(modules: Arc<Modules>) {
    let point = Router::new().route("/", post(point));

    let app = Router::new()
        .nest("/point", point)
        .with_state(modules)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 10));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn init_app() {
    tracing_subscriber::fmt::init();
}
