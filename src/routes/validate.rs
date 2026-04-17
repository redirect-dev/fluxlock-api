use axum::{extract::Json, response::Json as ResponseJson};
use serde::Deserialize;

use crate::engine::identity_validator::{validate_identity_logic, ValidationResult};

#[derive(Deserialize)]
pub struct IdentityInput {
    pub trust: f64,
    pub drift: f64,
    pub epoch_age: u64,
    pub epoch_valid: bool,
}

pub async fn validate_identity(
    Json(payload): Json<IdentityInput>,
) -> ResponseJson<ValidationResult> {
    let result = validate_identity_logic(
        payload.trust,
        payload.drift,
        payload.epoch_age,
        payload.epoch_valid,
    );

    ResponseJson(result)
}