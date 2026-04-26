use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::network_state::NetworkState;

#[derive(Deserialize)]
pub struct AccessRequest {
    pub id: u32,
}

#[derive(Serialize)]
pub struct AccessResponse {
    pub allowed: bool,
    pub confidence: f64,
    pub reason: String,

    pub status: String,
    pub epoch_age: u64,
    pub trust: f64,
    pub drift: f64,
}

pub async fn access(
    State(state): State<Arc<Mutex<NetworkState>>>,
    Json(payload): Json<AccessRequest>,
) -> Json<AccessResponse> {

    let mut state = state.lock().unwrap();

    // =========================
    // 🔍 FIND VALIDATOR
    // =========================
    let (allowed, confidence, reason, status, epoch_age, trust, drift) = {

        let validator = match state.validators.iter_mut().find(|v| v.id == payload.id) {
            Some(v) => v,
            None => {
                return Json(AccessResponse {
                    allowed: false,
                    confidence: 0.0,
                    reason: "validator not found".into(),
                    status: "unknown".into(),
                    epoch_age: 0,
                    trust: 0.0,
                    drift: 0.0,
                });
            }
        };

        // =========================
        // 🔐 HARD SECURITY GATES
        // =========================

        if !validator.chain_valid {
            return Json(AccessResponse {
                allowed: false,
                confidence: 0.0,
                reason: "identity chain invalid".into(),
                status: validator.status.clone(),
                epoch_age: validator.epoch_age,
                trust: validator.trust,
                drift: validator.drift,
            });
        }

        if validator.epoch_age < 120 {
            return Json(AccessResponse {
                allowed: false,
                confidence: 0.2,
                reason: "identity still maturing".into(),
                status: validator.status.clone(),
                epoch_age: validator.epoch_age,
                trust: validator.trust,
                drift: validator.drift,
            });
        }

        if !validator.network_accepted {
            return Json(AccessResponse {
                allowed: false,
                confidence: validator.confidence * 0.5,
                reason: "network has not accepted identity".into(),
                status: validator.status.clone(),
                epoch_age: validator.epoch_age,
                trust: validator.trust,
                drift: validator.drift,
            });
        }

        // ✅ ACCESS GRANTED
        (
            true,
            validator.confidence.clamp(0.0, 1.0),
            "access granted (consensus verified identity)".to_string(),
            validator.status.clone(),
            validator.epoch_age,
            validator.trust,
            validator.drift
        )
    };

    // =========================
    // 🔁 FEEDBACK (AFTER BORROW RELEASE)
    // =========================
    state.apply_access_feedback(
        payload.id,
        allowed,
        confidence,
    );

    Json(AccessResponse {
        allowed,
        confidence,
        reason,
        status,
        epoch_age,
        trust,
        drift,
    })
}