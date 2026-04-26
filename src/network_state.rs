use pqcrypto_traits::sign::PublicKey;
use serde::{Serialize, Deserialize};
use rand::Rng;

use crate::engine::identity_validator::{
    generate_identity,
    rotate_identity as crypto_rotate,
    verify_link,
    KEY_STORE,
};

// =========================
// 🔑 IDENTITY ENTRY
// =========================
#[derive(Clone, Serialize, Deserialize)]
pub struct IdentityEntry {
    pub public_key: Vec<u8>,
    pub signature: Option<Vec<u8>>,
    pub trust: f64,
}

// =========================
// 🧠 VALIDATOR
// =========================
#[derive(Clone, Serialize, Deserialize)]
pub struct Validator {
    pub id: u32,

    pub trust: f64,
    pub drift: f64,
    pub epoch_age: u64,
    pub status: String,

    pub recovery_timer: u64,

    pub identity_chain: Vec<IdentityEntry>,
    pub chain_valid: bool,

    pub continuity_score: f64,

    pub historical_drift: Vec<f64>,
    pub instability_score: f64,

    pub confidence: f64,
    pub behavior_score: f64,

    // 🔥 CONSENSUS VISIBILITY (RESTORED)
    pub local_valid: bool,
    pub global_valid: bool,
    pub network_accepted: bool,

    pub decision: String,
    pub weight: f64,
    pub reason: String,
}

// =========================
// 🌐 NETWORK STATE
// =========================
#[derive(Clone, Serialize, Deserialize)]
pub struct NetworkState {
    pub validators: Vec<Validator>,
    pub global_epoch: u64,
}

// =========================
// 🚀 INIT
// =========================
impl NetworkState {
    pub fn new() -> Self {
        let mut validators = Vec::new();

        for i in 0..20 {
            validators.push(Validator {
                id: i,
                trust: 80.0,
                drift: 5.0,
                epoch_age: 0,
                status: "healthy".into(),

                recovery_timer: 0,

                identity_chain: vec![IdentityEntry {
                    public_key: generate_identity(i),
                    signature: None,
                    trust: 80.0,
                }],
                chain_valid: true,

                continuity_score: 100.0,

                historical_drift: vec![],
                instability_score: 0.0,

                confidence: 0.5,
                behavior_score: 100.0,

                // 🔥 INIT CONSENSUS FLAGS
                local_valid: false,
                global_valid: false,
                network_accepted: false,

                decision: "WEIGHTED".into(),
                weight: 0.2,
                reason: "initial".into(),
            });
        }

        Self {
            validators,
            global_epoch: 0,
        }
    }
}

// =========================
// 🔁 MAIN LOOP
// =========================
impl NetworkState {
    pub fn tick(&mut self) {
        self.global_epoch += 1;

        self.simulate();
        self.consensus();
    }
}

// =========================
// ⚙️ EVOLUTION ENGINE
// =========================
impl NetworkState {

    fn simulate(&mut self) {
        let mut rng = rand::thread_rng();

        let snapshot: Vec<(u32, f64, f64)> =
            self.validators.iter().map(|v| (v.id, v.trust, v.drift)).collect();

        for v in &mut self.validators {

            v.epoch_age += 1;

            // 🔁 ROTATE IDENTITY (CORE NOVELTY)
            if v.epoch_age > 300 && v.recovery_timer == 0 {
                Self::rotate_identity(v);
                v.epoch_age = 0;
            }

            // 🔻 DRIFT DECAY
            v.drift *= 0.97;

            // 🌊 NETWORK INFLUENCE
            for (peer_id, _, peer_drift) in &snapshot {
                if *peer_id == v.id { continue; }

                if *peer_drift > 60.0 {
                    v.drift += (*peer_drift / 200.0) * rng.gen_range(1.0..3.0);
                }
            }

            // 📊 INSTABILITY MEMORY
            v.historical_drift.push(v.drift);
            if v.historical_drift.len() > 30 {
                v.historical_drift.remove(0);
            }

            v.instability_score =
                v.historical_drift.iter().sum::<f64>()
                / v.historical_drift.len() as f64;

            // 🎯 TRUST ADJUSTMENT
            if v.instability_score > 40.0 {
                v.trust *= 0.95;
            } else {
                v.trust += 0.1;
            }

            // ⚡ RANDOM EVENT
            if rng.gen_bool(0.002) {
                v.drift += rng.gen_range(10.0..20.0);
                v.trust -= 5.0;
                v.recovery_timer = 80;
            }

            // 🔁 RECOVERY STATE
            if v.recovery_timer > 0 {
                v.recovery_timer -= 1;
                v.status = "recovering".into();
            } else if v.drift > 80.0 {
                v.status = "attacked".into();
            } else if v.drift > 30.0 {
                v.status = "warning".into();
            } else {
                v.status = "healthy".into();
            }

            v.trust = v.trust.clamp(0.0, 100.0);
            v.drift = v.drift.clamp(0.0, 200.0);
        }
    }

    fn rotate_identity(v: &mut Validator) {
        let last = v.identity_chain.last().unwrap();

        let msg = format!("validator-{}", v.id).into_bytes();
        let sig = crypto_rotate(v.id, &msg);

        let new_pk = {
            let store = KEY_STORE.lock().unwrap();
            store.get(&v.id).unwrap().0.as_bytes().to_vec()
        };

        let valid = verify_link(&last.public_key, &msg, &sig);

        if !valid {
            v.chain_valid = false;
            v.continuity_score *= 0.5;
            return;
        }

        v.identity_chain.push(IdentityEntry {
            public_key: new_pk,
            signature: Some(sig),
            trust: v.trust,
        });

        if v.identity_chain.len() > 20 {
            v.identity_chain.remove(0);
        }

        v.chain_valid = true;
    }
}

// =========================
// 🗳️ CONSENSUS ENGINE (FIXED)
// =========================
impl NetworkState {

    fn consensus(&mut self) {

        let snapshot: Vec<(u32, f64)> =
            self.validators.iter().map(|v| (v.id, v.trust)).collect();

        for v in &mut self.validators {

            let mut score = 0.0;

            for (peer_id, peer_trust) in &snapshot {
                if *peer_id == v.id { continue; }

                let influence = peer_trust / 100.0;
                let stability = 1.0 - (v.drift / 150.0).min(1.0);

                score += influence * stability;
            }

            score /= snapshot.len() as f64;

            v.confidence = score.clamp(0.0, 1.0);

            // 🔥 RESTORED CONSENSUS SIGNALS
            v.local_valid = v.confidence > 0.5;
            v.global_valid = v.confidence > 0.7;

            v.network_accepted =
                v.local_valid &&
                v.global_valid &&
                v.chain_valid &&
                v.epoch_age >= 120;

            // 🎯 DECISION
            if !v.chain_valid {
                v.decision = "REJECT".into();
                v.weight = 0.0;

            } else if v.epoch_age < 120 {
                v.decision = "WEIGHTED".into();
                v.weight = 0.2;

            } else if v.network_accepted {
                v.decision = "ACCEPT".into();
                v.weight = v.confidence;

            } else {
                v.decision = "WEIGHTED".into();
                v.weight = v.confidence;
            }

            v.reason = "continuity + behavior + consensus".into();
        }
    }
}

// =========================
// ⚔️ ATTACKS
// =========================
impl NetworkState {

    pub fn spike_attack(&mut self, id: u32) {
        if let Some(v) = self.validators.iter_mut().find(|v| v.id == id) {
            v.drift += 60.0;
            v.trust *= 0.7;
            v.recovery_timer = 100;
        }
    }

    pub fn breach_attack(&mut self, id: u32) {
        if let Some(v) = self.validators.iter_mut().find(|v| v.id == id) {
            v.chain_valid = false;
            v.trust *= 0.3;
            v.drift += 80.0;
        }
    }

    pub fn network_attack(&mut self) {
        for v in &mut self.validators {
            v.drift += 25.0;
            v.trust *= 0.85;
        }
    }
}

// =========================
// 🔁 PRESSURE FEEDBACK
// =========================
impl NetworkState {

    pub fn apply_access_feedback(
        &mut self,
        id: u32,
        allowed: bool,
        confidence: f64,
    ) {
        if let Some(v) = self.validators.iter_mut().find(|v| v.id == id) {

            let is_maturing = v.epoch_age < 120;

            if allowed {
                v.trust += 0.4 * confidence;
                v.drift *= 0.96;
                v.behavior_score += 0.2;

            } else if !is_maturing {
                v.trust -= 1.0 * (1.0 - confidence);
                v.drift += 1.5 * (1.0 - confidence);
                v.behavior_score -= 0.3;

                if v.recovery_timer == 0 {
                    v.recovery_timer = 50;
                }
            }

            v.trust = v.trust.clamp(0.0, 100.0);
            v.drift = v.drift.clamp(0.0, 200.0);
            v.behavior_score = v.behavior_score.clamp(0.0, 100.0);
        }
    }
}