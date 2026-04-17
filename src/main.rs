mod state;
mod routes;

use axum::{
    routing::post,
    Router,
};

use routes::sign::sign;
use routes::verify::verify;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/sign", post(sign))
        .route("/verify", post(verify));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("🚀 Fluxlock PQ API running on http://127.0.0.1:3001");

    axum::serve(listener, app).await.unwrap();
}