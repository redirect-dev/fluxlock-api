use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose, Engine as _};
use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::SignedMessage;

use crate::network_state::NetworkState;
use crate::state::KEY_STORE;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub message: String,
    pub signature: String,
    pub validator_id: u32,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub authenticated: bool,
    pub signature_valid: bool,
    pub identity_valid: bool,
    pub allowed: bool,

    pub confidence: f64,
    pub reason: String,

    pub epoch_age: u64,
    pub trust: f64,
    pub drift: f64,
    pub status: String,
}

pub async fn auth_flow(
    State(state): State<Arc<Mutex<NetworkState>>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {

    // =========================
    // 🔐 VERIFY SIGNATURE
    // =========================
    let store = KEY_STORE.lock().unwrap();

    let (pk, _) = match store.get(&payload.validator_id) {
        Some(pair) => pair,
        None => {
            return Json(AuthResponse {
                authenticated: false,
                signature_valid: false,
                identity_valid: false,
                allowed: false,
                confidence: 0.0,
                reason: "validator not found".into(),
                epoch_age: 0,
                trust: 0.0,
                drift: 0.0,
                status: "unknown".into(),
            });
        }
    };

    let decoded = match general_purpose::STANDARD.decode(&payload.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(AuthResponse {
                authenticated: false,
                signature_valid: false,
                identity_valid: false,
                allowed: false,
                confidence: 0.0,
                reason: "invalid signature encoding".into(),
                epoch_age: 0,
                trust: 0.0,
                drift: 0.0,
                status: "invalid".into(),
            });
        }
    };

    let signed_msg = match dilithium2::SignedMessage::from_bytes(&decoded) {
        Ok(msg) => msg,
        Err(_) => {
            return Json(AuthResponse {
                authenticated: false,
                signature_valid: false,
                identity_valid: false,
                allowed: false,
                confidence: 0.0,
                reason: "invalid signed message".into(),
                epoch_age: 0,
                trust: 0.0,
                drift: 0.0,
                status: "invalid".into(),
            });
        }
    };

    let verify_result = dilithium2::open(&signed_msg, pk);
    let signature_valid = verify_result.is_ok();

    drop(store); // 🔥 release lock early

    if !signature_valid {
        return Json(AuthResponse {
            authenticated: false,
            signature_valid: false,
            identity_valid: false,
            allowed: false,
            confidence: 0.0,
            reason: "signature verification failed".into(),
            epoch_age: 0,
            trust: 0.0,
            drift: 0.0,
            status: "invalid".into(),
        });
    }

    // =========================
    // 🧠 LOAD STATE
    // =========================
    let mut state = state.lock().unwrap();

    let (allowed, confidence, reason, epoch_age, trust, drift, status, identity_valid) = {

        let validator = match state.validators.iter_mut().find(|v| v.id == payload.validator_id) {
            Some(v) => v,
            None => {
                return Json(AuthResponse {
                    authenticated: false,
                    signature_valid: true,
                    identity_valid: false,
                    allowed: false,
                    confidence: 0.0,
                    reason: "validator not found in state".into(),
                    epoch_age: 0,
                    trust: 0.0,
                    drift: 0.0,
                    status: "unknown".into(),
                });
            }
        };

        let identity_valid =
            validator.chain_valid &&
            validator.epoch_age >= 120;

        if !validator.chain_valid {
            return Json(AuthResponse {
                authenticated: false,
                signature_valid: true,
                identity_valid: false,
                allowed: false,
                confidence: 0.0,
                reason: "identity chain invalid".into(),
                epoch_age: validator.epoch_age,
                trust: validator.trust,
                drift: validator.drift,
                status: validator.status.clone(),
            });
        }

        if validator.epoch_age < 120 {
            return Json(AuthResponse {
                authenticated: false,
                signature_valid: true,
                identity_valid: false,
                allowed: false,
                confidence: 0.2,
                reason: "identity still maturing".into(),
                epoch_age: validator.epoch_age,
                trust: validator.trust,
                drift: validator.drift,
                status: validator.status.clone(),
            });
        }

        if !validator.network_accepted {
            return Json(AuthResponse {
                authenticated: false,
                signature_valid: true,
                identity_valid: true,
                allowed: false,
                confidence: validator.confidence * 0.5,
                reason: "network has not accepted identity".into(),
                epoch_age: validator.epoch_age,
                trust: validator.trust,
                drift: validator.drift,
                status: validator.status.clone(),
            });
        }

        (
            true,
            validator.confidence.clamp(0.0, 1.0),
            "authenticated (fluxlock verified identity)".to_string(),
            validator.epoch_age,
            validator.trust,
            validator.drift,
            validator.status.clone(),
            identity_valid,
        )
    };

    // =========================
    // 🔁 FEEDBACK
    // =========================
    state.apply_access_feedback(
        payload.validator_id,
        allowed,
        confidence,
    );

    Json(AuthResponse {
        authenticated: allowed,
        signature_valid: true,
        identity_valid,
        allowed,
        confidence,
        reason,
        epoch_age,
        trust,
        drift,
        status,
    })
}