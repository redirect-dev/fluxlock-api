use axum::{extract::State, Json};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::network_state::NetworkState;
use crate::engine::decision::evaluate_validator;

#[derive(Deserialize)]
pub struct EvaluateRequest {
    pub id: u32,
}

pub async fn evaluate(
    State(state): State<Arc<Mutex<NetworkState>>>,
    Json(payload): Json<EvaluateRequest>,
) -> Json<serde_json::Value> {
    let state = state.lock().unwrap();

    if let Some(v) = state.validators.iter().find(|v| v.id == payload.id) {
        let decision = evaluate_validator(v);

        Json(serde_json::json!({
            "decision": decision.decision,
            "weight": decision.weight,
            "status": decision.status,
            "reason": decision.reason
        }))
    } else {
        Json(serde_json::json!({
            "error": "validator not found"
        }))
    }
}