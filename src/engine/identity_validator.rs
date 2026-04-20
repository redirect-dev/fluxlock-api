pub struct ValidationResult {
    pub valid: bool,
    pub reason: String,
}

pub fn validate_identity_logic(
    trust: f64,
    drift: f64,
    epoch_age: u64,
    epoch_valid: bool,
    compromised: bool,
    network_accepted: bool, // 🔥 NEW
) -> ValidationResult {

    // 🔴 HARD FAILS
    if compromised {
        return ValidationResult {
            valid: false,
            reason: "identity compromised".into(),
        };
    }

    if !epoch_valid {
        return ValidationResult {
            valid: false,
            reason: "invalid identity chain".into(),
        };
    }

    // 🔥 CRITICAL ADDITION — MATURITY GATE
    if epoch_age < 120 {
        return ValidationResult {
            valid: false,
            reason: "identity too new (maturing)".into(),
        };
    }

    // 🔥 NETWORK MUST AGREE
    if !network_accepted {
        return ValidationResult {
            valid: false,
            reason: "network has not accepted identity".into(),
        };
    }

    // 🔴 INSTABILITY
    if drift > 80.0 {
        return ValidationResult {
            valid: false,
            reason: "unstable identity (high drift)".into(),
        };
    }

    // 🟡 RECOVERY ZONE
    if trust < 60.0 {
        return ValidationResult {
            valid: true,
            reason: "recovering identity".into(),
        };
    }

    // 🟢 HEALTHY
    ValidationResult {
        valid: true,
        reason: "identity stable and verified".into(),
    }
}