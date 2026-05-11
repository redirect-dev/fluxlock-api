use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct FluxIdentity {
    pub identity_id: String,

    // lifecycle
    pub created_epoch: u64,
    pub last_active_epoch: u64,
    pub session_count: u64,

    // trust + continuity
    pub trust_score: f64,
    pub continuity_score: f64,
    pub credential_depth: u64,

    // validator affinity
    pub bound_validator: u32,

    // behavioral memory
    pub successful_auths: u64,
    pub failed_auths: u64,
    pub recovery_events: u64,

    // adaptive state
    pub drift_score: f64,
    pub status: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IdentityRegistry {
    pub identities: HashMap<String, FluxIdentity>,
}

impl IdentityRegistry {

    pub fn new() -> Self {
        Self {
            identities: HashMap::new(),
        }
    }

    // =========================
    // 🆕 CREATE IDENTITY
    // =========================
    pub fn create_identity(
        &mut self,
        identity_id: String,
        validator_id: u32,
        current_epoch: u64,
    ) -> FluxIdentity {

        let identity = FluxIdentity {
            identity_id: identity_id.clone(),

            created_epoch: current_epoch,
            last_active_epoch: current_epoch,
            session_count: 0,

            trust_score: 50.0,
            continuity_score: 50.0,
            credential_depth: 1,

            bound_validator: validator_id,

            successful_auths: 0,
            failed_auths: 0,
            recovery_events: 0,

            drift_score: 0.0,
            status: "emerging".into(),
        };

        self.identities.insert(identity_id, identity.clone());

        identity
    }

    // =========================
    // 🔍 GET OR CREATE
    // =========================
    pub fn get_or_create(
        &mut self,
        identity_id: String,
        validator_id: u32,
        current_epoch: u64,
    ) -> FluxIdentity {

        if let Some(identity) = self.identities.get(&identity_id) {
            return identity.clone();
        }

        self.create_identity(
            identity_id,
            validator_id,
            current_epoch,
        )
    }

    // =========================
    // ✅ SUCCESS FEEDBACK
    // =========================
    pub fn successful_auth(
        &mut self,
        identity_id: &str,
        current_epoch: u64,
        confidence: f64,
    ) {

        if let Some(identity) = self.identities.get_mut(identity_id) {

            identity.last_active_epoch = current_epoch;

            identity.session_count += 1;
            identity.successful_auths += 1;

            identity.trust_score += 1.5 * confidence;
            identity.continuity_score += 0.8 * confidence;

            identity.drift_score *= 0.95;

            identity.credential_depth += 1;

            if identity.continuity_score > 80.0 {
                identity.status = "established".into();
            } else {
                identity.status = "maturing".into();
            }

            identity.trust_score =
                identity.trust_score.clamp(0.0, 100.0);

            identity.continuity_score =
                identity.continuity_score.clamp(0.0, 100.0);
        }
    }

    // =========================
    // ❌ FAILURE FEEDBACK
    // =========================
    pub fn failed_auth(
        &mut self,
        identity_id: &str,
        current_epoch: u64,
    ) {

        if let Some(identity) = self.identities.get_mut(identity_id) {

            identity.last_active_epoch = current_epoch;

            identity.failed_auths += 1;

            identity.trust_score *= 0.92;
            identity.continuity_score *= 0.96;

            identity.drift_score += 4.0;

            identity.recovery_events += 1;

            identity.status = "recovering".into();

            identity.trust_score =
                identity.trust_score.clamp(0.0, 100.0);

            identity.continuity_score =
                identity.continuity_score.clamp(0.0, 100.0);
        }
    }
}