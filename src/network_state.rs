use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Validator {
    pub id: u32,
    pub trust: f64,
    pub drift: f64,
    pub epoch_age: u64,
    pub status: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NetworkState {
    pub validators: Vec<Validator>,
}

impl NetworkState {
    pub fn new() -> Self {
        Self {
            validators: vec![
                Validator {
                    id: 0,
                    trust: 100.0,
                    drift: 0.0,
                    epoch_age: 0,
                    status: "healthy".to_string(),
                }
            ],
        }
    }
}