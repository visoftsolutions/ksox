#![allow(dead_code)]
use axum::Router;
use std::net::SocketAddr;

mod deserializer;
mod matching_engine;
mod serializer;
mod shutdown_signal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(deserializer::router())
        .merge(serializer::router());

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            shutdown_signal::listen().await;
        })
        .await
        .unwrap();
}
