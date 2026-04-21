use serde::Serialize;
use crate::network_state::Validator;

#[derive(Serialize)]
pub struct Decision {
    pub decision: String,   // ACCEPT | REJECT | WEIGHTED
    pub weight: f64,        // 0.0 → 1.0
    pub status: String,     // healthy | recovering | compromised
    pub reason: String,
}

pub fn evaluate_validator(v: &Validator) -> Decision {
    // =========================
    // 🔴 HARD IDENTITY GATE
    // =========================
    if !v.chain_valid {
        return Decision {
            decision: "REJECT".into(),
            weight: 0.0,
            status: "compromised".into(),
            reason: "identity chain invalid".into(),
        };
    }

    // =========================
    // ⏳ MATURITY GATE
    // =========================
    if v.epoch_age < 120 {
        return Decision {
            decision: "WEIGHTED".into(),
            weight: 0.2,
            status: "recovering".into(),
            reason: "identity still maturing".into(),
        };
    }

    // =========================
    // 🔥 RECOVERY MEMORY GATE
    // =========================
    if v.recovery_timer > 0 {
        let penalty = (v.recovery_timer as f64 / 200.0).min(1.0);

        return Decision {
            decision: "WEIGHTED".into(),
            weight: (v.trust / 100.0 * (1.0 - penalty)).clamp(0.1, 0.6),
            status: "recovering".into(),
            reason: "recent instability (recovery phase)".into(),
        };
    }

    // =========================
    // 🔴 EXTREME INSTABILITY
    // =========================
    if v.drift > 120.0 {
        return Decision {
            decision: "REJECT".into(),
            weight: 0.0,
            status: "attacked".into(),
            reason: "extreme instability".into(),
        };
    }

    // =========================
    // 🟡 PARTIAL TRUST ZONE
    // =========================
    if v.drift > 40.0 || v.trust < 60.0 {
        return Decision {
            decision: "WEIGHTED".into(),
            weight: (v.trust / 100.0 * 0.5).clamp(0.2, 0.7),
            status: "warning".into(),
            reason: "unstable identity".into(),
        };
    }

    // =========================
    // 🟢 FULL ACCEPTANCE
    // =========================
    Decision {
        decision: "ACCEPT".into(),
        weight: (v.trust / 100.0).clamp(0.6, 1.0),
        status: "healthy".into(),
        reason: "identity stable and trusted".into(),
    }
}