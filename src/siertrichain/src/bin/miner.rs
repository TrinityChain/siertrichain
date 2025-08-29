use siertrichain::mining::config::{MiningConfig, HardwareSelection};
use siertrichain::mining::miner::Miner;
use siertrichain::core::blockchain::Blockchain;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(author, version, about = "SierpinskiChain Miner CLI")]
struct Cli {
    /// Mining difficulty target
    #[clap(short, long, default_value = "1000")]
    difficulty: u64,

    /// Target triangle depth
    #[clap(short, long, default_value = "10")]
    depth: u32,

    /// Mining reward
    #[clap(short, long, default_value = "50")]
    reward: u64,

    /// Hardware selection (cpu/gpu)
    #[clap(short, long, value_enum, default_value = "cpu")]
    hardware: HardwareArg,
}

#[derive(ValueEnum, Clone, Debug)]
enum HardwareArg {
    Cpu,
    Gpu,
}

impl From<HardwareArg> for HardwareSelection {
    fn from(arg: HardwareArg) -> Self {
        match arg {
            HardwareArg::Cpu => HardwareSelection::Cpu,
            HardwareArg::Gpu => HardwareSelection::Gpu,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let config = MiningConfig {
        difficulty_target: cli.difficulty,
        target_triangle_depth: cli.depth,
        mining_reward: cli.reward,
        hardware: cli.hardware.into(),
    };

    let blockchain = Blockchain::new();
    let mut miner = Miner::new(config, blockchain);

    println!(
        "Starting miner with difficulty {}, depth {}, reward {}, hardware {:?}...",
        config.difficulty_target, config.target_triangle_depth, config.mining_reward, config.hardware
    );

    let block = miner.mine();
    println!("Mined new block: {:?}",