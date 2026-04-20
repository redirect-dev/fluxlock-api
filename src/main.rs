mod state;
mod network_state;
mod engine;   // 🔥 THIS LINE FIXES YOUR ERROR
mod routes;

use axum::{
    routing::{post, get},
    Router,
};
use std::sync::{Arc, Mutex};

use routes::sign::sign;
use routes::verify::verify;
use routes::validate::validate_identity;

use network_state::NetworkState;

#[tokio::main]
async fn main() {
    // =========================
    // 🧠 GLOBAL STATE (SOURCE OF TRUTH)
    // =========================
    let state = Arc::new(Mutex::new(NetworkState::new()));

    let app = Router::new()
        .route("/sign", post(sign))
        .route("/verify", post(verify))
        .route("/validate", post(validate_identity))
        .route("/state", get(get_state))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("🚀 Fluxlock API running on http://127.0.0.1:3001");

    axum::serve(listener, app).await.unwrap();
}

// =========================
// 🌐 STATE ENDPOINT
// =========================
use axum::{extract::State, Json};

async fn get_state(
    State(state): State<Arc<Mutex<NetworkState>>>,
) -> Json<NetworkState> {
    let state = state.lock().unwrap();
    Json(state.clone())
}