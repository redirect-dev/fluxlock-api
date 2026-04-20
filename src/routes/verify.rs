use axum::{Json, extract::Json as ExtractJson};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};

use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::SignedMessage;

// 🔥 KEY STORE
use crate::state::KEY_STORE;

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub message: String,
    pub signature: String,
    pub validator_id: u32,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub signature_valid: bool,
    pub identity_valid: bool,
}

pub async fn verify(
    ExtractJson(payload): ExtractJson<VerifyRequest>,
) -> Json<VerifyResponse> {

    // =========================
    // 🔐 FETCH PUBLIC KEY
    // =========================
    let store = KEY_STORE.lock().unwrap();

    let (pk, _) = match store.get(&payload.validator_id) {
        Some(pair) => pair,
        None => {
            return Json(VerifyResponse {
                signature_valid: false,
                identity_valid: false,
            });
        }
    };

    // =========================
    // 🔐 DECODE SIGNATURE
    // =========================
    let decoded = match general_purpose::STANDARD.decode(payload.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(VerifyResponse {
                signature_valid: false,
                identity_valid: false,
            });
        }
    };

    let signed_msg = match dilithium2::SignedMessage::from_bytes(&decoded) {
        Ok(msg) => msg,
        Err(_) => {
            return Json(VerifyResponse {
                signature_valid: false,
                identity_valid: false,
            });
        }
    };

    // =========================
    // 🔐 VERIFY SIGNATURE
    // =========================
    let result = dilithium2::open(&signed_msg, pk);

    let signature_valid = result.is_ok();

    // =========================
    // 🧠 IDENTITY VALIDITY (PLACEHOLDER LOGIC)
    // =========================
    // IMPORTANT:
    // This is NOT final — just prevents "always true" bug.
    // Real version will come from consensus / quorum.
    let identity_valid = signature_valid;

    Json(VerifyResponse {
        signature_valid,
        identity_valid,
    })
}