use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct IdentityInput {
    pub trust: f64,
    pub drift: f64,
    pub epoch_age: u64,
    pub epoch_valid: bool,
}

#[derive(Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub reason: String,
}

// 🔥 TEMP SAFE STUB
pub async fn validate_identity(
    Json(_payload): Json<IdentityInput>,
) -> Json<ValidationResult> {
    Json(ValidationResult {
        valid: true,
        reason: "stub".to_string(),
    })
}