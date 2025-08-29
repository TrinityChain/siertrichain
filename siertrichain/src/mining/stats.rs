use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MiningStats {
    pub hashrate: f64,
    pub geometric_complexity: f64,
    pub successful_subdivisions: u64,
}

impl MiningStats {
    pub fn new() -> Self {
        Self {
            hashrate: 0.0,
            geometric_complexity: 0.0,
            successful_subdivisions: 0,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
