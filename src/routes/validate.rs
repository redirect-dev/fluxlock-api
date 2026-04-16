use axum::{Json};
use serde::{Deserialize, Serialize};

use crate::engine::identity_validator::validate_identity_logic;

// ---------------- INPUT ----------------
#[derive(Deserialize)]
pub struct IdentityInput {
    pub identity: String,
    pub epoch: u64,
    pub current_epoch: u64,
    pub lineage_valid: bool,
    pub trust: f64,

    pub drift: Option<f64>,
    pub epoch_valid: Option<bool>,
}

// ---------------- OUTPUT ----------------
#[derive(Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub reason: String,
    pub confidence: f64,
}

// ---------------- ROUTE ----------------
pub async fn validate_identity(
    Json(payload): Json<IdentityInput>,
) -> Json<ValidationResult> {
    let result = validate_identity_logic(payload);
    Json(result)
}