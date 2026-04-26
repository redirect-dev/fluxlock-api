use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::network_state::NetworkState;
use crate::engine::decision::evaluate_validator;

#[derive(Deserialize)]
pub struct AccessRequest {
    pub id: u32,
}

#[derive(Serialize)]
pub struct AccessResponse {
    pub allowed: bool,
    pub confidence: f64,
    pub reason: String,
}

pub async fn access(
    State(state): State<Arc<Mutex<NetworkState>>>,
    Json(payload): Json<AccessRequest>,
) -> Json<AccessResponse> {

    let state = state.lock().unwrap();

    // =========================
    // 🔍 FIND VALIDATOR
    // =========================
    let validator = match state.validators.iter().find(|v| v.id == payload.id) {
        Some(v) => v,
        None => {
            return Json(AccessResponse {
                allowed: false,
                confidence: 0.0,
                reason: "validator not found".into(),
            });
        }
    };

    // =========================
    // 🧠 BASE DECISION (SOURCE OF TRUTH)
    // =========================
    let decision = evaluate_validator(validator);

    // =========================
    // 🔐 ACCESS RULES
    // =========================
    let allowed = decision.decision == "ACCEPT";

    // =========================
    // 📊 CONFIDENCE MODEL
    // =========================
    let mut confidence = decision.weight;

    // penalize recovery
    if validator.recovery_timer > 0 {
        confidence *= 0.7;
    }

    // penalize instability
    if validator.drift > 40.0 {
        confidence *= 0.6;
    }

    // hard fail if chain broken
    if !validator.chain_valid {
        return Json(AccessResponse {
            allowed: false,
            confidence: 0.0,
            reason: "identity chain invalid".into(),
        });
    }

    Json(AccessResponse {
        allowed,
        confidence: confidence.clamp(0.0, 1.0),
        reason: decision.reason,
    })
}