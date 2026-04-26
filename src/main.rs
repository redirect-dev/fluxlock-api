mod state;
mod network_state;
mod engine;
mod routes;

use axum::{
    routing::{post, get},
    Router,
};
use std::sync::{Arc, Mutex};

use routes::sign::sign;
use routes::verify::verify;
use routes::validate::validate_identity;
use routes::attack::{spike, breach, network};
use routes::evaluate::evaluate;
use routes::access::access; // 🔥 NEW

use network_state::NetworkState;

// 🔥 CORS
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
use routes::auth::auth_flow;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(NetworkState::new()));

    // =========================
    // 🔁 ENGINE LOOP
    // =========================
    let state_clone = state.clone();
    tokio::spawn(async move {
        loop {
            {
                let mut s = state_clone.lock().unwrap();
                s.tick();
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }
    });

    // =========================
    // 🌐 CORS CONFIG
    // =========================
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    // =========================
    // 🚀 ROUTER
    // =========================
    let app = Router::new()
        // 🔐 CRYPTO
        .route("/sign", post(sign))
        .route("/verify", post(verify))

        // 🧠 IDENTITY VALIDATION
        .route("/validate", post(validate_identity))

        // 🌐 STATE
        .route("/state", get(get_state))

        // 🔥 CORE PRODUCT
        .route("/evaluate", post(evaluate))
        .route("/access", post(access)) // 🔥 NEW (PILLAR 4)
        .route("/auth/flow", post(auth_flow))

        // ⚔ ATTACKS
        .route("/attack/spike", post(spike))
        .route("/attack/breach", post(breach))
        .route("/attack/network", post(network))

        .layer(cors)
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