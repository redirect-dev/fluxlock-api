use axum::{
    routing::get,
    Router,
    Json,
    extract::State,
};
use serde::Serialize;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;
use rand::Rng;
use tower_http::cors::{CorsLayer, Any};

#[derive(Clone, Serialize)]
struct Validator {
    id: usize,
    trust: f64,
    drift_score: f64,
    influence: f64,
    status: String,

    epoch_id: u64,
    epoch_age: u64,
    epoch_start_tick: u64,
    epoch_weight: f64,
    epoch_key: String,

    votes_for: f64,
    votes_against: f64,
    consensus_score: f64,
    valid: bool,
}

#[derive(Clone)]
struct AppState {
    validators: Arc<Mutex<Vec<Validator>>>,
    tick: Arc<Mutex<u64>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        validators: Arc::new(Mutex::new(init_validators())),
        tick: Arc::new(Mutex::new(0)),
    };

    // 🔁 SIM LOOP
    {
        let state = state.clone();
        tokio::spawn(async move {
            loop {
                {
                    let mut validators = state.validators.lock().unwrap();
                    let mut tick = state.tick.lock().unwrap();

                    *tick += 1;
                    let t = *tick;

                    let attacker =
                        ((t / 30) % validators.len() as u64) as usize;

                    // -------------------------
                    // STEP 1: BASE DYNAMICS
                    // -------------------------
                    for v in validators.iter_mut() {
                        v.epoch_age = t - v.epoch_start_tick;

                        if v.epoch_age > 20 {
                            v.epoch_id += 1;
                            v.epoch_start_tick = t;
                            v.epoch_age = 0;
                            v.epoch_key =
                                format!("{:x}", rand::thread_rng().gen::<u32>());
                        }

                        // 🔥 STRONGER ATTACK
                        if v.id == attacker {
                            v.status = "attacked".into();
                            v.trust -= 15.0;
                        } else {
                            v.status = "healthy".into();
                            v.trust += 0.5;
                        }

                        // 🔥 GLOBAL DECAY
                        v.trust -= 0.3;

                        v.trust = v.trust.clamp(0.0, 100.0);

                        v.drift_score =
                            rand::thread_rng().gen_range(-30.0..30.0);

                        v.influence = v.trust;
                        v.epoch_weight = v.trust / 100.0;
                    }

                    // -------------------------
                    // STEP 2: NETWORK EFFECT
                    // -------------------------
                    let snapshot = validators.clone();

                    for v in validators.iter_mut() {
                        let mut votes_for = 0.0;
                        let mut votes_against = 0.0;

                        for other in &snapshot {
                            if other.id == v.id {
                                continue;
                            }

                            // 🔥 TRUST-WEIGHTED VOTING
                            if other.status == "healthy" {
                                votes_for += other.trust;
                            } else {
                                votes_against += other.trust * 1.5;
                            }

                            // 🔥 CONTAGION
                            if other.status == "attacked" {
                                v.trust -= 0.2;
                            }
                        }

                        v.votes_for = votes_for;
                        v.votes_against = votes_against;

                        let total = votes_for + votes_against;

                        if total > 0.0 {
                            v.consensus_score = votes_for / total;
                        } else {
                            v.consensus_score = 0.0;
                        }

                        // 🔥 HARDER CONSENSUS
                        v.valid = v.consensus_score > 0.75;
                    }
                }

                sleep(Duration::from_millis(150)).await;
            }
        });
    }

    // ✅ CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/simulation", get(get_simulation))
        .layer(cors)
        .with_state(state);

    println!("🚀 http://127.0.0.1:8080");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_simulation(
    State(state): State<AppState>,
) -> Json<Vec<Validator>> {
    let validators = state.validators.lock().unwrap();
    Json(validators.clone())
}

fn init_validators() -> Vec<Validator> {
    let mut rng = rand::thread_rng();

    (0..20)
        .map(|i| Validator {
            id: i,
            trust: 100.0,
            drift_score: 0.0,
            influence: 100.0,
            status: "healthy".into(),

            epoch_id: 0,
            epoch_age: 0,
            epoch_start_tick: 0,
            epoch_weight: 1.0,
            epoch_key: format!("{:x}", rng.gen::<u32>()),

            votes_for: 0.0,
            votes_against: 0.0,
            consensus_score: 1.0,
            valid: true,
        })
        .collect()
}