use tiny_http::{Server, Response, Header};
use serde::Serialize;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Serialize, Clone)]
struct Validator {
    id: usize,
    trust: f64,
    influence: f64,
    status: String,
    behavior_score: f64,
    drift_score: f64,
    shock: f64,

    // NEW METRICS
    risk_score: f64,
    stability_score: f64,
    threat_level: String,
}

fn create_network() -> Vec<Validator> {
    let mut rng = rand::thread_rng();

    (0..20)
        .map(|i| Validator {
            id: i,
            trust: 90.0 + rng.gen_range(-3.0..3.0),
            influence: 65.0 + rng.gen_range(-5.0..5.0),
            status: "normal".to_string(),
            behavior_score: 92.0,
            drift_score: 0.0,
            shock: 0.0,

            risk_score: 0.0,
            stability_score: 0.0,
            threat_level: "low".to_string(),
        })
        .collect()
}

fn get_position(id: usize) -> (f64, f64) {
    let x = (id % 5) as f64;
    let y = (id / 5) as f64;
    (x, y)
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn update_network(validators: &mut Vec<Validator>) {
    let mut rng = rand::thread_rng();
    let mut attacker_collapsed = false;

    for v in validators.iter_mut() {
        let is_attacker = v.id == 19;
        let expected = 90.0;

        let actual = if is_attacker {
            if v.drift_score < 30.0 {
                rng.gen_range(85.0..95.0)
            } else if v.drift_score < 70.0 {
                rng.gen_range(60.0..80.0)
            } else {
                rng.gen_range(10.0..40.0)
            }
        } else {
            rng.gen_range(88.0..98.0)
        };

        v.behavior_score = (v.behavior_score * 0.85) + (actual * 0.15);

        let deviation = expected - v.behavior_score;

        if is_attacker {
            v.drift_score += deviation.max(0.0) * 0.8;
        } else {
            v.drift_score += deviation.max(0.0) * 0.2;
            v.drift_score *= 0.95;
        }

        if v.drift_score > 100.0 && is_attacker {
            v.status = "attacked".to_string();
            v.trust = 5.0;
            attacker_collapsed = true;
        } else if v.drift_score > 40.0 {
            v.status = "drifting".to_string();
        } else {
            v.status = "normal".to_string();
        }

        if v.drift_score > 20.0 {
            v.trust -= 4.0;
        } else {
            v.trust += 0.2;
        }

        v.trust = v.trust.clamp(0.0, 100.0);

        if v.trust < 30.0 {
            v.influence = 5.0;
        } else {
            v.influence = v.influence * 0.7 + v.trust * 0.3;
        }
    }

    // SHOCKWAVE
    if attacker_collapsed {
        let attacker_pos = get_position(19);

        let mut lost_influence = 0.0;

        for v in validators.iter_mut() {
            if v.id == 19 {
                lost_influence = v.influence;
                v.influence = 0.0;
            }
        }

        for v in validators.iter_mut() {
            if v.id != 19 {
                let pos = get_position(v.id);
                let dist = distance(pos, attacker_pos);

                let shock_strength = (4.0 - dist).max(0.0) * 5.0;

                v.shock = shock_strength;
                v.influence += lost_influence / 19.0;
            }
        }
    }

    for v in validators.iter_mut() {
        v.shock *= 0.85;

        // COMPUTED METRICS
        v.risk_score = v.drift_score + v.shock - v.trust * 0.5;
        v.stability_score = v.behavior_score - v.drift_score;

        v.threat_level = if v.drift_score > 80.0 {
            "critical".to_string()
        } else if v.drift_score > 40.0 {
            "elevated".to_string()
        } else {
            "low".to_string()
        };
    }
}

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();

    println!("🚀 Fluxlock API UPGRADED");

    let network = Arc::new(Mutex::new(create_network()));

    let sim = Arc::clone(&network);
    thread::spawn(move || loop {
        {
            let mut net = sim.lock().unwrap();
            update_network(&mut net);
        }
        thread::sleep(Duration::from_millis(1000));
    });

    for request in server.incoming_requests() {
        let url = request.url().to_string();

        let net = network.lock().unwrap();

        if url == "/simulation" {
            let json = serde_json::to_string(&*net).unwrap();

            let response = Response::from_string(json)
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());

            let _ = request.respond(response);
        }

        else if url.starts_with("/validator/") {
            let id = url.replace("/validator/", "").parse::<usize>().unwrap_or(0);
            let node = net.iter().find(|n| n.id == id);

            let json = serde_json::to_string(&node).unwrap();

            let _ = request.respond(
                Response::from_string(json)
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
            );
        }

        else if url == "/health" {
            let total = net.len();
            let avg_trust: f64 = net.iter().map(|n| n.trust).sum::<f64>() / total as f64;

            let response_data = serde_json::json!({
                "total": total,
                "avg_trust": avg_trust
            });

            let _ = request.respond(
                Response::from_string(response_data.to_string())
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
            );
        }

        else if url == "/risk" {
            let mut sorted = net.clone();
            sorted.sort_by(|a, b| b.risk_score.partial_cmp(&a.risk_score).unwrap());

            let _ = request.respond(
                Response::from_string(serde_json::to_string(&sorted).unwrap())
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
            );
        }

        else {
            let _ = request.respond(Response::from_string("Not Found"));
        }
    }
}