use axum::{Json, extract::Json as ExtractJson};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};

use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::SignedMessage;

// 🔥 SAME KEY STORE
use crate::state::KEY_STORE;

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub message: String,
    pub signature: String,
    pub validator_id: u32, // 🔥 NEW
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub valid: bool,
}

pub async fn verify(
    ExtractJson(payload): ExtractJson<VerifyRequest>,
) -> Json<VerifyResponse> {
    let store = KEY_STORE.lock().unwrap();

    let (pk, _) = match store.get(&payload.validator_id) {
        Some(pair) => pair,
        None => {
            return Json(VerifyResponse { valid: false });
        }
    };

    let decoded = general_purpose::STANDARD
        .decode(payload.signature)
        .unwrap();

    let signed_msg = dilithium2::SignedMessage::from_bytes(&decoded).unwrap();

    let result = dilithium2::open(&signed_msg, pk);

    Json(VerifyResponse {
        valid: result.is_ok(),
    })
}