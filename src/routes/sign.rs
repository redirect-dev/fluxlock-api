use axum::{Json, extract::Json as ExtractJson};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};

use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::SignedMessage;

// 🔥 GLOBAL KEY STORE
use crate::state::KEY_STORE;

#[derive(Deserialize)]
pub struct SignRequest {
    pub message: String,
    pub validator_id: u32, // 🔥 NEW
}

#[derive(Serialize)]
pub struct SignResponse {
    pub signature: String,
}

// 🔥 GET OR CREATE KEYPAIR
fn get_or_create_keypair(
    id: u32,
) -> (dilithium2::PublicKey, dilithium2::SecretKey) {
    let mut store = KEY_STORE.lock().unwrap();

    if let Some((pk, sk)) = store.get(&id) {
        return (pk.clone(), sk.clone());
    }

    let (pk, sk) = dilithium2::keypair();

    store.insert(id, (pk.clone(), sk.clone()));

    (pk, sk)
}

pub async fn sign(
    ExtractJson(payload): ExtractJson<SignRequest>,
) -> Json<SignResponse> {
    let (_pk, sk) = get_or_create_keypair(payload.validator_id);

    let msg = payload.message.as_bytes();

    let signed_msg = dilithium2::sign(msg, &sk);

    let encoded = general_purpose::STANDARD.encode(signed_msg.as_bytes());

    Json(SignResponse {
        signature: encoded,
    })
}