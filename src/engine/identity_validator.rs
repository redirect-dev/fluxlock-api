use crate::routes::validate::{IdentityInput, ValidationResult};

pub fn validate_identity_logic(input: IdentityInput) -> ValidationResult {
    let trust = input.trust;
    let drift = input.drift.unwrap_or(0.0);
    let epoch_valid = input.epoch_valid.unwrap_or(true);

    // -------------------------------
    // 1. HARD FAIL — tampered identity
    // -------------------------------
    if !epoch_valid {
        return ValidationResult {
            valid: false,
            reason: "invalid continuity (tampered identity)".to_string(),
            confidence: 0.1,
        };
    }

    // -------------------------------
    // 2. TRUE FAILURE (very rare)
    // -------------------------------
    if drift > 180.0 && trust < -80.0 {
        return ValidationResult {
            valid: false,
            reason: "identity irrecoverable".to_string(),
            confidence: 0.1,
        };
    }

    // -------------------------------
    // 3. RECOVERY FLOOR (CRITICAL)
    // -------------------------------
    if trust < 20.0 {
        return ValidationResult {
            valid: true,
            reason: "low trust (recovery enforced)".to_string(),
            confidence: 0.6,
        };
    }

    // -------------------------------
    // 4. HIGH INSTABILITY (ALLOW)
    // -------------------------------
    if drift > 60.0 {
        return ValidationResult {
            valid: true,
            reason: "high instability (recovery in progress)".to_string(),
            confidence: 0.5,
        };
    }

    // -------------------------------
    // 5. NORMAL RECOVERY
    // -------------------------------
    if trust < 60.0 {
        return ValidationResult {
            valid: true,
            reason: "recovering identity".to_string(),
            confidence: 0.7,
        };
    }

    // -------------------------------
    // 6. STABLE
    // -------------------------------
    ValidationResult {
        valid: true,
        reason: "identity valid (stable + continuous)".to_string(),
        confidence: (trust / 100.0).min(1.0),
    }
}