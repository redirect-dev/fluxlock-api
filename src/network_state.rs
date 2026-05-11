use serde::{Serialize, Deserialize};

use crate::identity::IdentityRegistry;

use crate::engine::identity_validator::{
    generate_identity,
    rotate_identity,
    verify_lineage,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Validator {
    pub id: u32,

    // consensus
    pub confidence: f64,
    pub trust: f64,
    pub drift: f64,

    // lifecycle
    pub epoch_age: u64,

    // validation state
    pub chain_valid: bool,
    pub network_accepted: bool,

    // compatibility
    pub recovery_timer: u64,
    pub peer_votes_valid: u32,
    pub peer_votes_invalid: u32,
    pub local_valid: bool,
    pub global_valid: bool,

    // lineage
    pub identity_chain: Vec<IdentityLink>,

    // health
    pub status: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IdentityLink {
    pub public_key: Vec<u8>,
    pub signature: Option<Vec<u8>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NetworkState {
    pub validators: Vec<Validator>,

    pub identities: IdentityRegistry,

    pub global_epoch: u64,
}

impl NetworkState {

    // =========================
    // 🌐 INIT
    // =========================
    pub fn new() -> Self {

        let mut validators = Vec::new();

        for i in 0..12 {

            // 🔥 GENESIS IDENTITY
            let genesis_key =
                generate_identity(i);

            let genesis_link =
                IdentityLink {
                    public_key: genesis_key,
                    signature: None,
                };

            validators.push(
                Validator {
                    id: i,

                    confidence: 0.92,
                    trust: 96.0,
                    drift: 2.0,

                    epoch_age: 180,

                    chain_valid: true,
                    network_accepted: true,

                    recovery_timer: 0,
                    peer_votes_valid: 8,
                    peer_votes_invalid: 1,

                    local_valid: true,
                    global_valid: true,

                    identity_chain:
                        vec![genesis_link],

                    status: "healthy".into(),
                }
            );
        }

        Self {
            validators,
            identities: IdentityRegistry::new(),
            global_epoch: 0,
        }
    }

    // =========================
    // 🔁 ENGINE LOOP
    // =========================
    pub fn tick(&mut self) {

        self.global_epoch += 1;

        for validator in &mut self.validators {

            validator.epoch_age += 1;

            validator.drift *= 0.995;

            validator.drift =
                validator.drift.clamp(0.0, 100.0);

            // 🔥 CONTINUOUS LINEAGE VERIFICATION
            validator.chain_valid =
                verify_lineage(
                    &validator.identity_chain,
                    validator.id,
                );

            // 🔴 FRACTURED STATE
            if !validator.chain_valid {

                validator.status =
                    "fractured".into();

                validator.network_accepted =
                    false;

                validator.local_valid =
                    false;

                validator.global_valid =
                    false;

                validator.trust *= 0.97;

                validator.confidence *= 0.96;

                validator.drift += 0.8;

                continue;
            }

            if validator.drift > 25.0 {

                validator.status =
                    "recovering".into();

                validator.confidence *= 0.992;
                validator.trust *= 0.996;

            } else {

                validator.status =
                    "healthy".into();

                validator.confidence += 0.0005;
                validator.trust += 0.02;
            }

            validator.confidence =
                validator.confidence.clamp(0.0, 1.0);

            validator.trust =
                validator.trust.clamp(0.0, 100.0);
        }

        // 🔥 IDENTITY EVOLUTION
        for identity in self.identities.identities.values_mut() {

            identity.continuity_score += 0.01;

            let idle_age =
                self.global_epoch
                    .saturating_sub(
                        identity.last_active_epoch
                    );

            if idle_age > 600 {

                identity.drift_score += 0.03;

                identity.trust_score *= 0.999;
            }

            if identity.status == "recovering"
                && identity.drift_score < 10.0 {

                identity.status =
                    "maturing".into();
            }
        }
    }

    // =========================
    // 🔁 ROTATE VALIDATOR IDENTITY
    // =========================
    pub fn evolve_identity(
        &mut self,
        validator_id: u32,
    ) {

        let validator =
            match self.validators
                .iter_mut()
                .find(|v| v.id == validator_id)
        {
            Some(v) => v,
            None => return,
        };

        let rotation_index =
            validator.identity_chain.len();

        let message =
            format!(
                "validator:{}:rotation:{}",
                validator_id,
                rotation_index,
            );

        let signature =
            rotate_identity(
                validator_id,
                message.as_bytes(),
            );

        let new_key =
            generate_identity(
                validator_id
            );

        validator.identity_chain.push(
            IdentityLink {

                public_key:
                    new_key,

                signature:
                    Some(signature),
            }
        );

        // 🔥 VERIFY ENTIRE LINEAGE
        validator.chain_valid =
            verify_lineage(
                &validator.identity_chain,
                validator_id,
            );

        // 🔴 FRACTURE DETECTION
        if !validator.chain_valid {

            validator.status =
                "fractured".into();

            validator.trust *= 0.5;

            validator.confidence *= 0.5;

            validator.network_accepted =
                false;

            validator.local_valid =
                false;

            validator.global_valid =
                false;
        }

        // prevent infinite memory growth
        if validator.identity_chain.len() > 64 {

            validator.identity_chain.remove(0);
        }
    }

    // =========================
    // ⚔ SPIKE ATTACK
    // =========================
    pub fn spike_attack(
        &mut self,
        id: u32,
    ) {

        if let Some(v) =
            self.validators.iter_mut()
                .find(|v| v.id == id)
        {

            v.drift += 12.0;

            v.trust *= 0.92;

            v.status =
                "recovering".into();
        }
    }

    // =========================
    // ☠ BREACH ATTACK
    // =========================
    pub fn breach_attack(
        &mut self,
        id: u32,
    ) {

        if let Some(v) =
            self.validators.iter_mut()
                .find(|v| v.id == id)
        {

            v.drift += 35.0;

            v.trust *= 0.7;

            v.confidence *= 0.75;

            v.chain_valid = false;

            v.network_accepted = false;

            v.local_valid = false;

            v.global_valid = false;

            v.status =
                "breached".into();
        }
    }

    // =========================
    // 🌊 NETWORK ATTACK
    // =========================
    pub fn network_attack(&mut self) {

        for v in &mut self.validators {

            v.drift += 6.0;

            v.trust *= 0.96;
        }
    }

    // =========================
    // 🔁 ACCESS FEEDBACK
    // =========================
    pub fn apply_access_feedback(
        &mut self,
        validator_id: u32,
        allowed: bool,
        confidence: f64,
    ) {

        if let Some(v) =
            self.validators.iter_mut()
                .find(|v| v.id == validator_id)
        {

            if allowed {

                v.confidence +=
                    0.01 * confidence;

                v.trust +=
                    0.4 * confidence;

                v.drift *= 0.97;

            } else {

                v.confidence *= 0.97;

                v.trust *= 0.985;

                v.drift += 2.0;
            }

            v.confidence =
                v.confidence.clamp(0.0, 1.0);

            v.trust =
                v.trust.clamp(0.0, 100.0);
        }
    }

    // =========================
    // 🧠 IDENTITIES
    // =========================
    pub fn get_or_create_identity(
        &mut self,
        identity_id: String,
        validator_id: u32,
    ) {

        self.identities.get_or_create(
            identity_id,
            validator_id,
            self.global_epoch,
        );
    }

    pub fn identity_success(
        &mut self,
        identity_id: &str,
        confidence: f64,
    ) {

        self.identities.successful_auth(
            identity_id,
            self.global_epoch,
            confidence,
        );
    }

    pub fn identity_failure(
        &mut self,
        identity_id: &str,
    ) {

        self.identities.failed_auth(
            identity_id,
            self.global_epoch,
        );
    }
}