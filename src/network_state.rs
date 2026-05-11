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

    // =========================
    // 🧠 CONSENSUS
    // =========================
    pub confidence: f64,
    pub trust: f64,
    pub drift: f64,

    // =========================
    // ⏳ LIFECYCLE
    // =========================
    pub epoch_age: u64,

    // =========================
    // ✅ VALIDATION
    // =========================
    pub chain_valid: bool,
    pub network_accepted: bool,

    // =========================
    // 🔄 RECOVERY
    // =========================
    pub recovery_timer: u64,
    pub rehabilitation_score: f64,
    pub rehabilitation_epochs: u64,

    // =========================
    // 🗳 CONSENSUS
    // =========================
    pub peer_votes_valid: u32,
    pub peer_votes_invalid: u32,
    pub local_valid: bool,
    pub global_valid: bool,

    // =========================
    // 🔗 LINEAGE
    // =========================
    pub identity_chain: Vec<IdentityLink>,

    // =========================
    // 🧠 MEMORY
    // =========================
    pub attack_history: u64,
    pub successful_recoveries: u64,

    pub resilience_score: f64,
    pub scar_level: f64,
    pub immune_response: f64,

    // =========================
    // 🌐 PRESSURE
    // =========================
    pub consensus_pressure: f64,
    pub instability_radius: f64,

    // =========================
    // 🌊 HEALING CONSENSUS
    // =========================
    pub stabilization_power: f64,
    pub rehabilitation_votes: u32,
    pub fracture_severity: f64,
    pub continuity_anchor_strength: f64,

    // =========================
    // 🌐 EPOCH SYSTEM
    // =========================
    pub current_epoch: u64,
    pub inherited_trust: f64,
    pub lineage_stability: f64,
    pub epoch_rotations: u64,
    pub rebirth_count: u64,
    pub last_epoch_transition: u64,

    // =========================
    // 🔥 STATE
    // =========================
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

            let genesis_key =
                generate_identity(i);

            let genesis_link =
                IdentityLink {

                    public_key:
                        genesis_key,

                    signature:
                        None,
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
                    rehabilitation_score: 100.0,
                    rehabilitation_epochs: 0,

                    peer_votes_valid: 8,
                    peer_votes_invalid: 1,

                    local_valid: true,
                    global_valid: true,

                    identity_chain:
                        vec![genesis_link],

                    attack_history: 0,

                    successful_recoveries: 0,

                    resilience_score: 100.0,

                    scar_level: 0.0,

                    immune_response: 1.0,

                    consensus_pressure: 0.0,

                    instability_radius: 0.0,

                    stabilization_power: 1.0,

                    rehabilitation_votes: 0,

                    fracture_severity: 0.0,

                    continuity_anchor_strength: 100.0,

                    current_epoch: 0,

                    inherited_trust: 96.0,

                    lineage_stability: 100.0,

                    epoch_rotations: 0,

                    rebirth_count: 0,

                    last_epoch_transition: 0,

                    status:
                        "healthy".into(),
                }
            );
        }

        Self {

            validators,

            identities:
                IdentityRegistry::new(),

            global_epoch: 0,
        }
    }

    // =========================
    // 🔁 ENGINE LOOP
    // =========================
    pub fn tick(&mut self) {

        self.global_epoch += 1;

        // =========================
        // 🌐 NETWORK EPOCH ROTATION
        // =========================
        if self.global_epoch % 1200 == 0 {

            let validator_ids: Vec<u32> =
                self.validators
                    .iter()
                    .map(|v| v.id)
                    .collect();

            for id in validator_ids {

                self.perform_epoch_rotation(id);
            }
        }

        let avg_drift =
            self.validators
                .iter()
                .map(|v| v.drift)
                .sum::<f64>()
                / self.validators.len() as f64;

        // =========================
        // 🌊 DISTRIBUTED STABILIZATION
        // =========================
        let stabilization_pool =
            self.validators
                .iter()
                .filter(|v|
                    v.network_accepted
                    && v.trust > 70.0
                    && v.drift < 20.0
                )
                .map(|v|
                    v.stabilization_power
                    * (v.resilience_score / 100.0)
                    * (v.trust / 100.0)
                )
                .sum::<f64>();

        // =========================
        // 🗳 GLOBAL REHABILITATION QUORUM
        // =========================
        let rehabilitation_quorum =
            self.validators
                .iter()
                .filter(|peer|

                    peer.network_accepted
                    && peer.trust > 65.0
                    && peer.drift < 25.0
                )
                .count() as u32;

        // =========================
        // 🌐 VALIDATOR LOOP
        // =========================
        for validator in
            &mut self.validators
        {

            validator.epoch_age += 1;

            validator.consensus_pressure =
                avg_drift * 0.12;

            validator.instability_radius =
                validator.drift * 0.35;

            // =========================
            // 🌊 CONSENSUS HEALING
            // =========================
            let stabilization_effect =
                stabilization_pool * 0.002;

            if validator.drift > 10.0 {

                validator.drift -=
                    stabilization_effect;

                validator.trust +=
                    stabilization_effect * 0.08;

                validator.rehabilitation_score +=
                    stabilization_effect * 0.12;
            }

            // =========================
            // 🧠 IMMUNE RESPONSE
            // =========================
            let immune_factor =
                validator.immune_response
                    .max(1.0);

            let resilience_factor =
                validator.resilience_score
                    / 100.0;

            // =========================
            // 🌊 DRIFT DECAY
            // =========================
            let adaptive_decay =
                0.992
                + (resilience_factor * 0.004)
                + (immune_factor * 0.002);

            validator.drift *= adaptive_decay;

            validator.drift =
                validator
                    .drift
                    .clamp(0.0, 100.0);

            // =========================
            // 🔗 VERIFY LINEAGE
            // =========================
            validator.chain_valid =
                verify_lineage(
                    &validator.identity_chain,
                    validator.id,
                );

            // =========================
            // 🔴 FRACTURED
            // =========================
            if !validator.chain_valid {

                validator.status =
                    "fractured".into();

                validator.network_accepted =
                    false;

                validator.local_valid =
                    false;

                validator.global_valid =
                    false;

                validator.recovery_timer += 1;

                validator.trust *= 0.997;

                validator.confidence *= 0.996;

                validator.drift += 0.2;

                validator.scar_level += 0.1;

                validator.fracture_severity += 0.4;

                validator.continuity_anchor_strength *= 0.995;

                continue;
            }

            // =========================
            // 🔥 REHABILITATION
            // =========================
            if !validator.network_accepted {

                validator.status =
                    "rehabilitating".into();

                validator.rehabilitation_epochs += 1;

                validator.rehabilitation_score +=
                    stabilization_effect * 0.5;

                if validator.drift < 15.0 {

                    validator.rehabilitation_score +=
                        0.25
                        + (validator.immune_response * 0.03);

                    validator.fracture_severity *= 0.992;

                } else {

                    validator.rehabilitation_score -= 0.35;

                    validator.fracture_severity += 0.02;
                }

                validator.rehabilitation_score =
                    validator
                        .rehabilitation_score
                        .clamp(0.0, 100.0);

                validator.rehabilitation_votes =
                    rehabilitation_quorum;

                // =========================
                // 🔁 REINSTATEMENT
                // =========================
                if validator.rehabilitation_score > 70.0
                    && validator.rehabilitation_epochs > 90
                    && validator.rehabilitation_votes >= 5
                {

                    validator.network_accepted = true;

                    validator.local_valid = true;

                    validator.global_valid = true;

                    validator.status =
                        "reinstated".into();

                    validator.successful_recoveries += 1;

                    validator.resilience_score += 5.0;

                    validator.immune_response += 0.15;

                    validator.recovery_timer = 0;

                    validator.fracture_severity *= 0.5;
                }

                continue;
            }

            // =========================
            // 🟡 RECOVERING
            // =========================
            if validator.drift > 25.0 {

                validator.status =
                    "recovering".into();

                validator.confidence *= 0.993;

                validator.trust *= 0.997;

            } else {

                validator.status =
                    "healthy".into();

                validator.confidence +=
                    0.0004
                    * immune_factor;

                validator.trust +=
                    0.01
                    * resilience_factor;

                validator.continuity_anchor_strength += 0.02;

                validator.stabilization_power += 0.001;
            }

            // =========================
            // 🛡 IMMUNE HARDENING
            // =========================
            if validator.attack_history > 3
                && validator.drift < 10.0
            {

                validator.status =
                    "immune".into();

                validator.immune_response += 0.002;

                validator.resilience_score += 0.003;

                validator.continuity_anchor_strength += 0.05;

                validator.stabilization_power += 0.003;
            }

            // =========================
            // 🧠 SCAR DECAY
            // =========================
            validator.scar_level *= 0.999;

            validator.fracture_severity *= 0.996;

            validator.confidence =
                validator
                    .confidence
                    .clamp(0.0, 1.0);

            validator.trust =
                validator
                    .trust
                    .clamp(0.0, 100.0);

            validator.resilience_score =
                validator
                    .resilience_score
                    .clamp(0.0, 200.0);

            validator.immune_response =
                validator
                    .immune_response
                    .clamp(1.0, 10.0);

            validator.continuity_anchor_strength =
                validator
                    .continuity_anchor_strength
                    .clamp(0.0, 200.0);

            validator.stabilization_power =
                validator
                    .stabilization_power
                    .clamp(0.0, 10.0);
        }

        // =========================
        // 🔗 IDENTITY EVOLUTION
        // =========================
        for identity in
            self.identities
                .identities
                .values_mut()
        {

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
                && identity.drift_score < 10.0
            {

                identity.status =
                    "maturing".into();
            }
        }
    }

    // =========================
    // 🌐 EPOCH ROTATION
    // =========================
    pub fn perform_epoch_rotation(
        &mut self,
        validator_id: u32,
    ) {

        let global_epoch =
            self.global_epoch;

        let should_rotate =
        {
            let validator =
                match self.validators
                    .iter_mut()
                    .find(|v| v.id == validator_id)
            {
                Some(v) => v,
                None => return,
            };

            if !validator.chain_valid {

                return;
            }

            if !validator.network_accepted {

                return;
            }

            validator.inherited_trust =
                (validator.inherited_trust * 0.7)
                + (validator.trust * 0.3);

            validator.lineage_stability +=
                validator.continuity_anchor_strength * 0.01;

            validator.lineage_stability =
                validator
                    .lineage_stability
                    .clamp(0.0, 200.0);

            validator.current_epoch += 1;

            validator.epoch_rotations += 1;

            validator.last_epoch_transition =
                global_epoch;

            true
        };

        if should_rotate {

            self.evolve_identity(
                validator_id
            );
        }

        if let Some(validator) =
            self.validators
                .iter_mut()
                .find(|v| v.id == validator_id)
        {

            validator.drift *= 0.92;

            validator.trust += 2.0;

            validator.confidence += 0.015;

            validator.status =
                "epoch_transition".into();
        }
    }

    // =========================
    // 🔁 EVOLVE IDENTITY
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

        if validator.trust < 40.0
            || validator.drift > 45.0
            || !validator.network_accepted
            || validator.fracture_severity > 10.0
        {

            return;
        }

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

        validator.chain_valid =
            verify_lineage(
                &validator.identity_chain,
                validator.id,
            );

        if !validator.chain_valid {

            validator.status =
                "fractured".into();

            validator.network_accepted =
                false;

            validator.local_valid =
                false;

            validator.global_valid =
                false;

            validator.rehabilitation_score =
                25.0;

            validator.rehabilitation_epochs =
                0;

            validator.fracture_severity += 3.0;
        }

        if validator.identity_chain.len() > 64 {

            validator.identity_chain.remove(0);
        }
    }

    // =========================
    // ⚡ SPIKE
    // =========================
    pub fn spike_attack(
        &mut self,
        id: u32,
    ) {

        if let Some(v) =
            self.validators
                .iter_mut()
                .find(|v| v.id == id)
        {

            let resistance =
                v.immune_response
                    * (v.resilience_score / 100.0);

            let impact =
                (12.0 / resistance)
                    .max(2.0);

            v.drift += impact;

            v.trust *= 0.94;

            v.attack_history += 1;

            v.scar_level += 0.3;

            v.fracture_severity += 0.2;

            v.status =
                "recovering".into();
        }
    }

    // =========================
    // ☠ BREACH
    // =========================
    pub fn breach_attack(
        &mut self,
        id: u32,
    ) {

        if let Some(v) =
            self.validators
                .iter_mut()
                .find(|v| v.id == id)
        {

            let resistance =
                v.immune_response
                    * (v.resilience_score / 100.0);

            let impact =
                (35.0 / resistance)
                    .max(8.0);

            v.drift += impact;

            v.trust *= 0.72;

            v.confidence *= 0.75;

            v.attack_history += 1;

            v.scar_level += 1.2;

            v.fracture_severity += 2.0;

            v.continuity_anchor_strength *= 0.92;

            v.network_accepted = false;

            v.local_valid = false;

            v.global_valid = false;

            v.status =
                "quarantined".into();

            v.rehabilitation_score =
                20.0;

            v.rehabilitation_epochs =
                0;
        }
    }

    // =========================
    // 🌊 NETWORK ATTACK
    // =========================
    pub fn network_attack(
        &mut self
    ) {

        for v in
            &mut self.validators
        {

            let resistance =
                v.immune_response
                    * (v.resilience_score / 100.0);

            let impact =
                (6.0 / resistance)
                    .max(1.0);

            v.drift += impact;

            v.trust *= 0.97;

            v.attack_history += 1;

            v.scar_level += 0.15;

            v.fracture_severity += 0.15;

            v.continuity_anchor_strength *= 0.998;
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
            self.validators
                .iter_mut()
                .find(|v| v.id == validator_id)
        {

            if allowed {

                v.confidence +=
                    0.01 * confidence;

                v.trust +=
                    0.25 * confidence;

                v.drift *= 0.96;

                v.fracture_severity *= 0.985;

                v.continuity_anchor_strength += 0.25;

                if !v.network_accepted {

                    v.rehabilitation_score += 1.2;
                }

            } else {

                v.confidence *= 0.98;

                v.trust *= 0.99;

                v.drift += 1.0;

                v.fracture_severity += 0.1;
            }

            v.confidence =
                v.confidence
                    .clamp(0.0, 1.0);

            v.trust =
                v.trust
                    .clamp(0.0, 100.0);
        }
    }

    // =========================
    // 🧠 IDENTITY MEMORY
    // =========================
    pub fn get_or_create_identity(
        &mut self,
        identity_id: String,
        validator_id: u32,
    ) {

        self.identities
            .get_or_create(
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

        self.identities
            .successful_auth(
                identity_id,
                self.global_epoch,
                confidence,
            );
    }

    pub fn identity_failure(
        &mut self,
        identity_id: &str,
    ) {

        self.identities
            .failed_auth(
                identity_id,
                self.global_epoch,
            );
    }
}