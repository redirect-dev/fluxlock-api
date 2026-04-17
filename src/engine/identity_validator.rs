// identity_validator.rs
// 🔥 UPDATED FOR INSTANT COMPROMISE MODEL

pub struct ValidationResult {
    pub valid: bool,
    pub reason: String,
}

pub fn validate_identity_logic(
    trust: f64,
    drift: f64,
    epoch_age: u64,
    epoch_valid: bool,
    compromised: bool, // 🔥 NEW
) -> ValidationResult {

    // =========================
    // 🔴 HARD FAILURE — COMPROMISED
    // =========================
    if compromised {
        return ValidationResult {
            valid: false,
            reason: "identity compromised (key breach detected)".to_string(),
        };
    }

    // =========================
    // 🔴 HARD FAILURE — INVALID EPOCH
    // =========================
    if !epoch_valid {
        return ValidationResult {
            valid: false,
            reason: "invalid epoch (tampered identity)".to_string(),
        };
    }

    // =========================
    // 🔴 HIGH DRIFT = REJECT
    // =========================
    if drift > 90.0 {
        return ValidationResult {
            valid: false,
            reason: "critical instability (drift too high)".to_string(),
        };
    }

    // =========================
    // 🟡 RECOVERY STATES
    // =========================
    if drift > 70.0 {
        return ValidationResult {
            valid: true,
            reason: "high instability (recovery in progress)".to_string(),
        };
    }

    if trust < 30.0 {
        return ValidationResult {
            valid: true,
            reason: "low trust (recovery enforced)".to_string(),
        };
    }

    if trust < 60.0 {
        return ValidationResult {
            valid: true,
            reason: "recovering identity".to_string(),
        };
    }

    // =========================
    // 🟢 HEALTHY
    // =========================
    ValidationResult {
        valid: true,
        reason: "identity valid (stable + continuous)".to_string(),
    }
}