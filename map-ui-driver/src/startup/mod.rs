use crate::{modules::Modules, routes::point::point};
use axum::{routing::post, Router};
use hyper::header::CONTENT_TYPE;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub async fn startup(modules: Arc<Modules>) {
    let point = Router::new().route("/", post(point));

    // corsの設定
    let app = Router::new()
        .nest("/point", point)
        .with_state(modules)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn init_app() {
    tracing_subscriber::fmt::init();
}
