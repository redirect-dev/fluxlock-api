use serde::{Serialize, Deserialize};
use rand::Rng;
use pqcrypto_traits::sign::PublicKey;

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
    pub identity_age: u64,
    pub last_chain_break: u64,

    pub historical_drift: Vec<f64>,
    pub instability_score: f64,

    // 🔥 CONSENSUS (FIXED)
    pub peer_votes_valid: u32,
    pub peer_votes_invalid: u32,
    pub network_accepted: bool,
    pub local_valid: bool,
    pub global_valid: bool,
    pub consensus_confidence: f64,

    // PRODUCT
    pub confidence: f64,
    pub risk_score: f64,
    pub behavior_score: f64,

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
    pub epoch_pressure: f64,
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
                identity_age: 0,
                last_chain_break: 0,

                historical_drift: vec![],
                instability_score: 0.0,

                peer_votes_valid: 0,
                peer_votes_invalid: 0,
                network_accepted: false,
                local_valid: false,
                global_valid: false,
                consensus_confidence: 0.0,

                confidence: 0.5,
                risk_score: 0.2,
                behavior_score: 100.0,

                decision: "WEIGHTED".into(),
                weight: 0.2,
                reason: "initial".into(),
            });
        }

        Self {
            validators,
            global_epoch: 0,
            epoch_pressure: 0.0,
        }
    }
}

// =========================
// 🔁 MAIN LOOP
// =========================
impl NetworkState {
    pub fn tick(&mut self) {
        self.global_epoch += 1;

        // 🔥 SLOWER PRESSURE (FIX)
        self.epoch_pressure = (self.global_epoch as f64 / 2000.0).min(1.0);

        self.simulate_step();
        self.apply_consensus();
    }
}

// =========================
// ⚙️ EVOLUTION
// =========================
impl NetworkState {
    fn simulate_step(&mut self) {
        let mut rng = rand::thread_rng();

        let snapshot: Vec<(u32, f64, f64)> =
            self.validators.iter().map(|v| (v.id, v.trust, v.drift)).collect();

        let avg_trust =
            snapshot.iter().map(|(_, t, _)| t).sum::<f64>() / snapshot.len() as f64;

        for v in &mut self.validators {
            v.epoch_age += 1;
            v.identity_age += 1;

            // 🔁 CONTROLLED ROTATION (FIX)
            if (v.epoch_age > 350 ||
                (self.epoch_pressure > 0.8 && v.drift < 20.0))
                && v.recovery_timer == 0
            {
                Self::rotate_identity(v);
                v.epoch_age = 0;
            }

            v.drift *= 0.96;

            for (peer_id, _, peer_drift) in &snapshot {
                if *peer_id == v.id { continue; }

                if *peer_drift > 60.0 {
                    let influence = (*peer_drift / 200.0)
                        * rng.gen_range(2.0..5.0);
                    v.drift += influence;
                }
            }

            v.historical_drift.push(v.drift);
            if v.historical_drift.len() > 50 {
                v.historical_drift.remove(0);
            }

            v.instability_score =
                v.historical_drift.iter().sum::<f64>()
                / v.historical_drift.len() as f64;

            if v.trust < avg_trust {
                v.trust += 0.2;
            }

            if v.instability_score > 40.0 {
                v.trust *= 0.9;
            }

            if rng.gen_bool(0.002) {
                v.drift += rng.gen_range(10.0..25.0);
                v.trust -= rng.gen_range(5.0..10.0);
                v.recovery_timer = 100;
            }

            if v.recovery_timer > 0 {
                v.status = "recovering".into();
                v.recovery_timer -= 1;
            } else if v.drift > 80.0 {
                v.status = "attacked".into();
            } else if v.drift > 25.0 {
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

        let msg = format!("validator-{}-epoch", v.id).into_bytes();
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
// 🗳️ CONSENSUS (FIXED)
// =========================
impl NetworkState {
    fn apply_consensus(&mut self) {
        let snapshot: Vec<(u32, f64)> =
            self.validators.iter().map(|v| (v.id, v.trust)).collect();

        for v in &mut self.validators {
            let mut valid = 0;
            let mut invalid = 0;

            for (peer_id, peer_trust) in &snapshot {
                if *peer_id == v.id { continue; }

                let confidence =
                    (v.trust / 100.0) * 0.6 +
                    (1.0 - (v.drift / 150.0).min(1.0)) * 0.4;

                if confidence > 0.5 + (*peer_trust / 100.0) * 0.1 {
                    valid += 1;
                } else {
                    invalid += 1;
                }
            }

            let total = valid + invalid;
            let ratio = if total > 0 {
                valid as f64 / total as f64
            } else {
                0.0
            };

            let drift_factor = 1.0 - (v.drift / 150.0).min(1.0);
            let trust_factor = v.trust / 100.0;

            v.confidence = (ratio * trust_factor * drift_factor).clamp(0.0, 1.0);
            v.risk_score = (1.0 - v.confidence).clamp(0.0, 1.0);

            // 🔥 FIX: REAL CONSENSUS
            v.local_valid = v.confidence > 0.5;
            v.global_valid = ratio > 0.66;
            v.network_accepted = v.local_valid && v.global_valid;
            v.consensus_confidence = ratio;

            if v.confidence < 0.3 {
                v.decision = "REJECT".into();
                v.weight = 0.0;
            } else if v.confidence < 0.7 {
                v.decision = "WEIGHTED".into();
                v.weight = v.confidence;
            } else {
                v.decision = "ACCEPT".into();
                v.weight = v.confidence;
            }

            v.reason = "adaptive trust evaluation".into();

            v.peer_votes_valid = valid;
            v.peer_votes_invalid = invalid;
        }
    }
}

// =========================
// ⚔️ ATTACKS
// =========================
impl NetworkState {

    pub fn spike_attack(&mut self, id: u32) {
        if let Some(v) = self.validators.iter_mut().find(|v| v.id == id) {
            v.drift += 50.0;
            v.trust *= 0.7;
            v.recovery_timer = 120;
            v.status = "attacked".into();
        }
    }

    pub fn breach_attack(&mut self, id: u32) {
        if let Some(v) = self.validators.iter_mut().find(|v| v.id == id) {
            v.chain_valid = false;
            v.trust *= 0.3;
            v.drift += 80.0;
            v.status = "compromised".into();
        }
    }

    pub fn network_attack(&mut self) {
        for v in &mut self.validators {
            v.drift += 30.0;
            v.trust *= 0.85;
            if v.recovery_timer == 0 {
                v.recovery_timer = 80;
            }
        }
    }
}