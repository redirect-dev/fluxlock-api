use axum::Json;
use serde::{Deserialize, Serialize};

use crate::engine::identity_validator::validate_identity_logic;

#[derive(Deserialize)]
pub struct IdentityInput {
    pub trust: f64,
    pub drift: f64,
    pub epoch_age: u64,
    pub epoch_valid: bool,
    pub compromised: bool,
    pub network_accepted: bool, // 🔥 ADD THIS
}

#[derive(Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub reason: String,
}

pub async fn validate_identity(
    Json(payload): Json<IdentityInput>,
) -> Json<ValidationResult> {

    let result = validate_identity_logic(
        payload.trust,
        payload.drift,
        payload.epoch_age,
        payload.epoch_valid,
        payload.compromised,
        payload.network_accepted, // 🔥 PASS IT
    );

    Json(ValidationResult {
        valid: result.valid,
        reason: result.reason,
    })
}