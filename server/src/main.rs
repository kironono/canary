use std::net::SocketAddr;

use axum::{
    prelude::{get, RoutingDsl},
    route,
};
use tracing::Level;

async fn root() -> &'static str {
    tracing::debug!("called");
    "Hello World!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let app = route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("listening on {}", addr);

    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
