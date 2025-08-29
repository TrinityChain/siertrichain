use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MiningConfig {
    pub difficulty_target: u64,
    pub target_triangle_depth: u32,
    pub mining_reward: u64,
    pub hardware: HardwareSelection,
}

#[derive(Debug, Deserialize)]
pub enum HardwareSelection {
    Cpu,
    Gpu,
}

impl MiningConfig {
    pub fn from_toml(path: &str) -> Result<Self, toml::de::Error> {
        let toml_str = std::fs::read_to_string(path).unwrap();
        toml::from_str(&toml_str)
    }
}
