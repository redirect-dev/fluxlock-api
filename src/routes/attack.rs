use axum::{extract::State, Json};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::network_state::NetworkState;

#[derive(Deserialize)]
pub struct AttackRequest {
    pub id: u32,
}

// ⚡ Spike
pub async fn spike(
    State(state): State<Arc<Mutex<NetworkState>>>,
    Json(payload): Json<AttackRequest>,
) -> &'static str {
    let mut state = state.lock().unwrap();
    state.spike_attack(payload.id);
    "ok"
}

// ☠️ Breach
pub async fn breach(
    State(state): State<Arc<Mutex<NetworkState>>>,
    Json(payload): Json<AttackRequest>,
) -> &'static str {
    let mut state = state.lock().unwrap();
    state.breach_attack(payload.id);
    "ok"
}

// 🌊 Network
pub async fn network(
    State(state): State<Arc<Mutex<NetworkState>>>,
) -> &'static str {
    let mut state = state.lock().unwrap();
    state.network_attack();
    "ok"
}