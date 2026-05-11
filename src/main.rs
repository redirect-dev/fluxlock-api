mod state;
mod network_state;
mod identity;
mod engine;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};

use std::sync::{Arc, Mutex};

use routes::sign::sign;
use routes::verify::verify;
use routes::validate::validate_identity;
use routes::attack::{spike, breach, network};
use routes::evaluate::evaluate;
use routes::access::access;
use routes::auth::auth_flow;

// 🔥 NEW
use routes::identity_create::create_identity;

use network_state::NetworkState;

// 🌐 CORS
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

#[tokio::main]
async fn main() {

    // =========================
    // 🌐 GLOBAL NETWORK STATE
    // =========================
    let state = Arc::new(
        Mutex::new(
            NetworkState::new()
        )
    );

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

            tokio::time::sleep(
                std::time::Duration::from_millis(300)
            ).await;
        }
    });

    // =========================
    // 🌐 CORS CONFIG
    // =========================
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
        ])
        .allow_headers(Any);

    // =========================
    // 🚀 ROUTER
    // =========================
    let app = Router::new()

        // 🔐 CRYPTO
        .route("/sign", post(sign))
        .route("/verify", post(verify))

        // 🧠 VALIDATION
        .route("/validate", post(validate_identity))

        // 🌊 NETWORK STATE
        .route("/state", get(get_state))

        // 🔥 IDENTITY
        .route(
            "/identity/create",
            post(create_identity)
        )

        // 🔥 CORE PRODUCT
        .route("/evaluate", post(evaluate))
        .route("/access", post(access))
        .route("/auth/flow", post(auth_flow))

        // ⚔ ATTACK SIMULATION
        .route("/attack/spike", post(spike))
        .route("/attack/breach", post(breach))
        .route("/attack/network", post(network))

        .layer(cors)

        .with_state(state.clone());

    // =========================
    // 🌐 SERVER START
    // =========================
    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:3001"
    )
    .await
    .unwrap();

    println!(
        "🚀 Fluxlock API running on http://127.0.0.1:3001"
    );

    println!(
        "🧠 Persistent Flux Identities enabled"
    );

    println!(
        "🆕 Identity Genesis endpoint enabled"
    );

    axum::serve(listener, app)
        .await
        .unwrap();
}

// =========================
// 🌐 STATE ENDPOINT
// =========================
use axum::{
    extract::State,
    Json,
};

async fn get_state(
    State(state): State<Arc<Mutex<NetworkState>>>,
) -> Json<NetworkState> {

    let state = state.lock().unwrap();

    Json(state.clone())
}