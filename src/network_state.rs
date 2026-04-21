use serde::{Serialize, Deserialize};
use rand::Rng;

// =========================
// 🔑 IDENTITY ENTRY
// =========================
#[derive(Clone, Serialize, Deserialize)]
pub struct IdentityEntry {
    pub fingerprint: String,
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

    // 🔥 CONSENSUS
    pub peer_votes_valid: u32,
    pub peer_votes_invalid: u32,
    pub network_accepted: bool,
    pub local_valid: bool,
    pub global_valid: bool,

    // 🔑 IDENTITY
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

                peer_votes_valid: 0,
                peer_votes_invalid: 0,
                network_accepted: false,
                local_valid: false,
                global_valid: false,

                identity_chain: vec![IdentityEntry {
                    fingerprint: format!("genesis-{}", i),
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

            // 🔁 ROTATION TRIGGER
            if v.epoch_age > 150 {
                Self::rotate_identity(v);
                v.epoch_age = 0;
            }

            v.drift *= 0.97;

            if v.trust < 100.0 {
                v.trust += 0.2;
            }

            if rng.gen_bool(0.02) {
                v.drift += rng.gen_range(10.0..40.0);
                v.trust -= rng.gen_range(5.0..15.0);
            }

            v.trust = v.trust.clamp(0.0, 100.0);
            v.drift = v.drift.clamp(0.0, 200.0);

            v.status = if v.drift > 80.0 {
                "attacked".into()
            } else if v.drift > 30.0 {
                "warning".into()
            } else {
                "healthy".into()
            };
        }
    }

    fn rotate_identity(v: &mut Validator) {
        let mut rng = rand::thread_rng();

        let new_key = format!("key-{}", rng.gen::<u64>());

        // 🔥 SIMULATED FAILURE CHANCE
        if rng.gen_bool(0.1) {
            v.chain_valid = false;
            return;
        }

        v.identity_chain.push(IdentityEntry {
            fingerprint: new_key,
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
                    confidence *= 0.2; // 🔥 penalize broken identity
                }

                let mut threshold = 0.55 + (peer.trust / 100.0) * 0.25;

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

            let network_accepted = ratio > 0.6;
            let local_valid = ratio > 0.5;

            let global_valid =
                network_accepted &&
                v.chain_valid &&
                v.drift < 15.0 &&
                v.trust > 70.0;

            v.peer_votes_valid = valid;
            v.peer_votes_invalid = invalid;
            v.network_accepted = network_accepted;
            v.local_valid = local_valid;
            v.global_valid = global_valid;
        }
    }

    fn compute_confidence(v: &Validator) -> f64 {
        let trust_score = v.trust / 100.0;
        let drift_penalty = (v.drift / 150.0).min(1.0);

        trust_score * 0.7 + (1.0 - drift_penalty) * 0.3
    }
}