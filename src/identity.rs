use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// =========================
// 🔗 IDENTITY PROOF
// =========================
#[derive(Clone, Serialize, Deserialize)]
pub struct IdentityProof {

    pub epoch: u64,

    pub validator_id: u32,

    pub trust: f64,

    pub continuity: f64,

    pub previous_hash: String,

    pub proof_hash: String,
}

// =========================
// 🧬 FLUX IDENTITY
// =========================
#[derive(Clone, Serialize, Deserialize)]
pub struct FluxIdentity {

    pub identity_id: String,

    // lifecycle
    pub created_epoch: u64,
    pub last_active_epoch: u64,
    pub session_count: u64,

    // trust
    pub trust_score: f64,
    pub continuity_score: f64,

    // validator affinity
    pub bound_validator: u32,

    // auth memory
    pub successful_auths: u64,
    pub failed_auths: u64,
    pub recovery_events: u64,

    // adaptive state
    pub drift_score: f64,
    pub status: String,

    // 🔥 lineage
    pub credential_depth: u64,

    pub lineage:
        Vec<IdentityProof>,
}

// =========================
// 🌐 REGISTRY
// =========================
#[derive(Clone, Serialize, Deserialize)]
pub struct IdentityRegistry {

    pub identities:
        HashMap<String, FluxIdentity>,
}

impl IdentityRegistry {

    pub fn new() -> Self {

        Self {
            identities:
                HashMap::new(),
        }
    }

    // =========================
    // 🔐 HASH GENERATOR
    // =========================
    fn build_hash(
        epoch: u64,
        validator_id: u32,
        trust: f64,
        continuity: f64,
        previous_hash: &str,
    ) -> String {

        format!(
            "{:x}",
            md5::compute(
                format!(
                    "{}:{}:{}:{}:{}",
                    epoch,
                    validator_id,
                    trust,
                    continuity,
                    previous_hash
                )
            )
        )
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

        let genesis_hash =
            Self::build_hash(
                current_epoch,
                validator_id,
                50.0,
                50.0,
                "GENESIS",
            );

        let genesis_proof =
            IdentityProof {

                epoch:
                    current_epoch,

                validator_id,

                trust: 50.0,

                continuity: 50.0,

                previous_hash:
                    "GENESIS".into(),

                proof_hash:
                    genesis_hash,
            };

        let identity = FluxIdentity {

            identity_id:
                identity_id.clone(),

            created_epoch:
                current_epoch,

            last_active_epoch:
                current_epoch,

            session_count: 0,

            trust_score: 50.0,
            continuity_score: 50.0,

            bound_validator:
                validator_id,

            successful_auths: 0,
            failed_auths: 0,
            recovery_events: 0,

            drift_score: 0.0,

            status:
                "genesis".into(),

            credential_depth: 1,

            lineage:
                vec![genesis_proof],
        };

        self.identities.insert(
            identity_id,
            identity.clone(),
        );

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

        if let Some(identity) =
            self.identities.get(&identity_id)
        {
            return identity.clone();
        }

        self.create_identity(
            identity_id,
            validator_id,
            current_epoch,
        )
    }

    // =========================
    // 🧠 LIFECYCLE ENGINE
    // =========================
    fn update_stage(
        identity: &mut FluxIdentity,
    ) {

        if identity.drift_score > 40.0 {

            identity.status =
                "quarantined".into();

            return;
        }

        if identity.recovery_events > 0 {

            identity.status =
                "recovering".into();

            return;
        }

        let continuity =
            identity.continuity_score;

        if continuity < 58.0 {

            identity.status =
                "emerging".into();

        } else if continuity < 70.0 {

            identity.status =
                "stabilizing".into();

        } else if continuity < 85.0 {

            identity.status =
                "established".into();

        } else {

            identity.status =
                "sovereign".into();
        }
    }

    // =========================
    // 🔗 APPEND LINEAGE
    // =========================
    fn append_lineage(
        identity: &mut FluxIdentity,
        epoch: u64,
    ) {

        let previous_hash =
            identity
                .lineage
                .last()
                .map(|p| p.proof_hash.clone())
                .unwrap_or(
                    "GENESIS".into()
                );

        let proof_hash =
            Self::build_hash(
                epoch,
                identity.bound_validator,
                identity.trust_score,
                identity.continuity_score,
                &previous_hash,
            );

        identity.lineage.push(
            IdentityProof {

                epoch,

                validator_id:
                    identity.bound_validator,

                trust:
                    identity.trust_score,

                continuity:
                    identity.continuity_score,

                previous_hash,

                proof_hash,
            }
        );
    }

    // =========================
    // ✅ SUCCESS
    // =========================
    pub fn successful_auth(
        &mut self,
        identity_id: &str,
        current_epoch: u64,
        confidence: f64,
    ) {

        if let Some(identity) =
            self.identities.get_mut(identity_id)
        {

            identity.last_active_epoch =
                current_epoch;

            identity.session_count += 1;

            identity.successful_auths += 1;

            identity.trust_score +=
                1.5 * confidence;

            identity.continuity_score +=
                0.8 * confidence;

            identity.drift_score *= 0.92;

            identity.credential_depth += 1;

            if identity.recovery_events > 0 {

                identity.recovery_events -= 1;
            }

            identity.trust_score =
                identity
                    .trust_score
                    .clamp(0.0, 100.0);

            identity.continuity_score =
                identity
                    .continuity_score
                    .clamp(0.0, 100.0);

            Self::update_stage(
                identity
            );

            // 🔥 lineage extension
            Self::append_lineage(
                identity,
                current_epoch,
            );
        }
    }

    // =========================
    // ❌ FAILURE
    // =========================
    pub fn failed_auth(
        &mut self,
        identity_id: &str,
        current_epoch: u64,
    ) {

        if let Some(identity) =
            self.identities.get_mut(identity_id)
        {

            identity.last_active_epoch =
                current_epoch;

            identity.failed_auths += 1;

            identity.trust_score *= 0.90;

            identity.continuity_score *= 0.95;

            identity.drift_score += 5.0;

            identity.recovery_events += 1;

            identity.trust_score =
                identity
                    .trust_score
                    .clamp(0.0, 100.0);

            identity.continuity_score =
                identity
                    .continuity_score
                    .clamp(0.0, 100.0);

            Self::update_stage(
                identity
            );

            // 🔥 lineage extension
            Self::append_lineage(
                identity,
                current_epoch,
            );
        }
    }
}