use serde::Serialize;

#[derive(Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub reason: String,
}

/// Core Fluxlock identity validation logic
pub fn validate_identity_logic(
    trust: f64,
    drift: f64,
    epoch_age: u64,
    epoch_valid: bool,
) -> ValidationResult {
    // =========================
    // 🔴 HARD FAIL CONDITIONS
    // =========================

    // Identity tampering / invalid epoch
    if !epoch_valid {
        return ValidationResult {
            valid: false,
            reason: "epoch integrity failure (tampered identity)".to_string(),
        };
    }

    // Extreme instability — drift overrides everything
    if drift >= 95.0 {
        return ValidationResult {
            valid: false,
            reason: "critical instability (drift too high)".to_string(),
        };
    }

    // =========================
    // 🟡 ROTATION PROBATION WINDOW
    // =========================

    if epoch_age <= 2 {
        if drift > 70.0 {
            return ValidationResult {
                valid: false,
                reason: "new identity unstable (rotation probation)".to_string(),
            };
        }

        return ValidationResult {
            valid: true,
            reason: "new identity stabilizing (probation window)".to_string(),
        };
    }

    // =========================
    // 🔶 HIGH INSTABILITY ZONE
    // =========================

    if drift > 85.0 {
        return ValidationResult {
            valid: false,
            reason: "identity unstable (high drift)".to_string(),
        };
    }

    if drift > 60.0 {
        return ValidationResult {
            valid: true,
            reason: "high instability (recovery in progress)".to_string(),
        };
    }

    // =========================
    // 🔵 LOW TRUST RECOVERY MODE
    // =========================

    if trust < 30.0 {
        return ValidationResult {
            valid: true,
            reason: "low trust (recovery enforced)".to_string(),
        };
    }

    // =========================
    // 🟢 NORMAL OPERATION
    // =========================

    if trust > 70.0 && drift < 50.0 {
        return ValidationResult {
            valid: true,
            reason: "identity valid (stable + continuous)".to_string(),
        };
    }

    if trust > 50.0 {
        return ValidationResult {
            valid: true,
            reason: "identity valid (moderate confidence)".to_string(),
        };
    }

    // =========================
    // 🔻 DEFAULT FALLBACK
    // =========================

    ValidationResult {
        valid: false,
        reason: "identity confidence insufficient".to_string(),
    }
}