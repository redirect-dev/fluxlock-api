use serde::{Serialize, Deserialize};
use rand::Rng;

use crate::engine::identity_validator::{
    generate_identity,
    rotate_identity as crypto_rotate,
    verify_link,
    KEY_STORE,
};

// =========================
// 🔑 IDENTITY ENTRY (REAL CRYPTO)
// =========================
#[derive(Clone, Serialize, Deserialize)]
pub struct IdentityEntry {
    pub public_key: Vec<u8>,
    pub signature: Option<Vec<u8>>, // signed by previous key
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

    // CONSENSUS
    pub peer_votes_valid: u32,
    pub peer_votes_invalid: u32,
    pub network_accepted: bool,
    pub local_valid: bool,
    pub global_valid: bool,

    // IDENTITY
    pub identity_chain: Vec<IdentityEntry>,
    pub chain_valid: bool,
}

// =========================
// 🌐 NETWORK STATE
// =========================
#[derive(Clone, Serialize, Deserialize)]
pub struct NetworkState {
    pub validators: Vec<Validator>,
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

                peer_votes_valid: 0,
                peer_votes_invalid: 0,
                network_accepted: false,
                local_valid: false,
                global_valid: false,

                identity_chain: vec![IdentityEntry {
                    public_key: generate_identity(i),
                    signature: None,
                    trust: 80.0,
                }],
                chain_valid: true,
            });
        }

        Self { validators }
    }
}

// =========================
// 🔁 MAIN LOOP
// =========================
impl NetworkState {
    pub fn tick(&mut self) {
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

        for v in &mut self.validators {
            v.epoch_age += 1;

            // 🔁 CRYPTO IDENTITY ROTATION
            if v.epoch_age > 150 {
                Self::rotate_identity(v);
                v.epoch_age = 0;
            }

            if v.recovery_timer > 0 {
                v.recovery_timer -= 1;
            }

            v.drift *= 0.985;

            if v.trust < 100.0 {
                v.trust += 0.1;
            }

            if rng.gen_bool(0.02) {
                v.drift += rng.gen_range(10.0..40.0);
                v.trust -= rng.gen_range(5.0..15.0);
                v.recovery_timer = 120;
            }

            v.trust = v.trust.clamp(0.0, 100.0);
            v.drift = v.drift.clamp(0.0, 200.0);

            v.status = if v.recovery_timer > 0 {
                "recovering".into()
            } else if v.drift > 80.0 {
                "attacked".into()
            } else if v.drift > 30.0 {
                "warning".into()
            } else {
                "healthy".into()
            };
        }
    }

    fn rotate_identity(v: &mut Validator) {
        let last = v.identity_chain.last().unwrap();

        let new_message = format!("validator-{}-epoch", v.id).into_bytes();

        // 🔐 SIGN WITH OLD KEY + GENERATE NEW KEY
        let signature = crypto_rotate(v.id, &new_message);

        let new_pk = {
            let store = KEY_STORE.lock().unwrap();
            store.get(&v.id).unwrap().0.as_bytes().to_vec()
        };

        // ✅ VERIFY CHAIN LINK
        let valid = verify_link(
            &last.public_key,
            &new_message,
            &signature,
        );

        if !valid {
            v.chain_valid = false;
            v.recovery_timer = 200;
            return;
        }

        v.identity_chain.push(IdentityEntry {
            public_key: new_pk,
            signature: Some(signature),
            trust: v.trust,
        });

        v.chain_valid = true;
    }
}

// =========================
// 🗳️ CONSENSUS
// =========================
impl NetworkState {
    fn apply_consensus(&mut self) {
        let snapshot = self.validators.clone();

        for v in &mut self.validators {
            let mut valid = 0;
            let mut invalid = 0;

            for peer in &snapshot {
                if peer.id == v.id {
                    continue;
                }

                let mut confidence = Self::compute_confidence(v);

                if !v.chain_valid {
                    confidence *= 0.1;
                }

                if v.recovery_timer > 0 {
                    confidence *= 0.5;
                }

                let threshold = 0.55 + (peer.trust / 100.0) * 0.25;

                if confidence > threshold {
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

            v.network_accepted = ratio > 0.6;
            v.local_valid = ratio > 0.5;

            v.global_valid =
                v.network_accepted &&
                v.chain_valid &&
                v.recovery_timer == 0 &&
                v.drift < 15.0 &&
                v.trust > 70.0;

            v.peer_votes_valid = valid;
            v.peer_votes_invalid = invalid;
        }
    }

    fn compute_confidence(v: &Validator) -> f64 {
        let trust_score = v.trust / 100.0;
        let drift_penalty = (v.drift / 150.0).min(1.0);

        trust_score * 0.7 + (1.0 - drift_penalty) * 0.3
    }
}

// =========================
// ⚔️ ATTACKS
// =========================
impl NetworkState {
    pub fn spike_attack(&mut self, id: u32) {
        if let Some(v) = self.validators.iter_mut().find(|v| v.id == id) {
            v.drift += 80.0;
            v.trust -= 40.0;
            v.recovery_timer = 120;
        }
    }

    pub fn breach_attack(&mut self, id: u32) {
        if let Some(v) = self.validators.iter_mut().find(|v| v.id == id) {
            v.drift = 150.0;
            v.trust *= 0.3;
            v.chain_valid = false;
            v.recovery_timer = 200;
            v.status = "attacked".into();
        }
    }

    pub fn network_attack(&mut self) {
        for v in &mut self.validators {
            v.drift += 20.0;
            v.trust -= 10.0;
            v.recovery_timer = 80;
        }
    }
}