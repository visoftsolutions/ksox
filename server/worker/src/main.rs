use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

mod api;
mod shutdown_signal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .nest("/private", api::private::router())
        .nest("/public", api::public::router());

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

async fn root() -> &'static str {
    "Hello, World!"
}
