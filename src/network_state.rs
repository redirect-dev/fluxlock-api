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

    pub peer_votes_valid: u32,
    pub peer_votes_invalid: u32,
    pub network_accepted: bool,
    pub local_valid: bool,
    pub global_valid: bool,

    pub identity_chain: Vec<IdentityEntry>,
    pub chain_valid: bool,

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

                decision: "WEIGHTED".into(),
                weight: 0.2,
                reason: "initial".into(),
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

        let snapshot: Vec<(u32, f64, f64)> =
            self.validators.iter().map(|v| (v.id, v.trust, v.drift)).collect();

        let avg_trust: f64 =
            snapshot.iter().map(|(_, t, _)| t).sum::<f64>() / snapshot.len() as f64;

        for v in &mut self.validators {
            v.epoch_age += 1;

            // =========================
            // ☣️ QUARANTINE
            // =========================
            if !v.chain_valid {
                v.status = "quarantined".into();
                v.recovery_timer += 1;

                v.trust += 0.25;
                v.drift *= 0.95;

                if v.recovery_timer > 80 {
                    Self::force_rebuild_identity(v);
                }

                if v.trust > 25.0 && v.drift < 15.0 {
                    v.chain_valid = true;
                    v.status = "recovering".into();
                    v.recovery_timer = 120;
                }

                continue;
            }

            // =========================
            // 🔁 ROTATION
            // =========================
            if v.epoch_age > (350 + (v.id as u64 * 5)) && v.recovery_timer == 0 {
                Self::rotate_identity(v);
                v.epoch_age = 0;
            }

            if v.recovery_timer > 0 {
                v.recovery_timer -= 1;
            }

            // =========================
            // 🌊 DRIFT DECAY
            // =========================
            v.drift *= 0.96;

            // =========================
            // 🌊 PROPAGATION
            // =========================
            let mut external_drift = 0.0;

            for (peer_id, _peer_trust, peer_drift) in &snapshot {
                if *peer_id == v.id {
                    continue;
                }

                if *peer_drift > 60.0 {
                    let influence =
                        (*peer_drift / 200.0) * (1.0 - (v.trust / 100.0));

                    external_drift += influence * 8.0;
                }
            }

            v.drift += external_drift;

            // =========================
            // 🤝 NETWORK SUPPORT
            // =========================
            if v.trust < avg_trust * 0.5 {
                v.trust += 0.3;
            }

            // =========================
            // 🔥 RECOVERY
            // =========================
            if v.recovery_timer > 0 {
                v.trust += 0.5;
                v.drift *= 0.90;
            } else {
                if v.trust < 50.0 {
                    v.trust += 0.4;
                    v.drift *= 0.94;
                } else if v.trust < 90.0 {
                    v.trust += 0.2;
                } else {
                    v.trust += 0.05;
                }
            }

            // =========================
            // 🎲 RANDOM DISTURBANCE
            // =========================
            if !(v.trust > 90.0 && v.drift < 5.0) {
                if rng.gen_bool(0.002) {
                    v.drift += rng.gen_range(10.0..25.0);
                    v.trust -= rng.gen_range(5.0..10.0);
                    v.recovery_timer = 120;
                }
            }

            // =========================
            // 🧭 STATUS
            // =========================
            if v.recovery_timer > 0 {
                v.status = "recovering".into();
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
            v.recovery_timer = 0;
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

    fn force_rebuild_identity(v: &mut Validator) {
        v.identity_chain.clear();

        v.identity_chain.push(IdentityEntry {
            public_key: generate_identity(v.id),
            signature: None,
            trust: 20.0,
        });

        v.trust = 20.0;
        v.drift = 10.0;
        v.epoch_age = 0;
    }
}

// =========================
// 🗳️ CONSENSUS (RESTORED)
// =========================
impl NetworkState {
    fn apply_consensus(&mut self) {
        let snapshot: Vec<(u32, f64)> =
            self.validators.iter().map(|v| (v.id, v.trust)).collect();

        for v in &mut self.validators {
            let mut valid = 0;
            let mut invalid = 0;

            for (peer_id, peer_trust) in &snapshot {
                if *peer_id == v.id {
                    continue;
                }

                let mut confidence =
                    (v.trust / 100.0) * 0.7 + (1.0 - (v.drift / 150.0).min(1.0)) * 0.3;

                if !v.chain_valid {
                    confidence *= 0.2;
                }

                if v.recovery_timer > 0 {
                    confidence *= 0.85;
                }

                let drift_penalty = (v.drift / 100.0).min(0.5);
                let threshold = 0.55 + drift_penalty + (*peer_trust / 100.0) * 0.15;

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

            v.network_accepted =
                ratio > 0.6 && v.drift < 25.0 && v.recovery_timer == 0;

            v.local_valid = ratio > 0.5;

            v.global_valid =
                v.network_accepted &&
                v.chain_valid &&
                v.drift < 15.0 &&
                v.trust > 75.0;

            v.peer_votes_valid = valid;
            v.peer_votes_invalid = invalid;

            if !v.network_accepted {
                v.decision = "REJECT".into();
                v.weight = 0.0;
            } else if v.epoch_age < 120 || v.drift > 15.0 {
                v.decision = "WEIGHTED".into();
                v.weight = 0.2;
            } else {
                v.decision = "ACCEPT".into();
                v.weight = (v.trust / 100.0) * (1.0 - (v.drift / 100.0));
            }

            v.reason = "consensus evaluation".into();
        }
    }
}

// =========================
// ⚔️ ATTACKS (RESTORED)
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
            v.recovery_timer = 0;
        }
    }

    pub fn network_attack(&mut self) {
        for v in &mut self.validators {
            v.drift += 20.0;
            v.trust -= 10.0;
            v.recovery_timer = 100;
        }
    }
}