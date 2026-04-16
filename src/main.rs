use axum::{routing::post, Router};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

mod routes;
mod engine;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/validate-identity", post(routes::validate::validate_identity))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Fluxlock API running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}