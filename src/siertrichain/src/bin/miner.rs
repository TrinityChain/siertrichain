use siertrichain::mining::config::MiningConfig;
use siertrichain::mining::miner::Miner;
use siertrichain::core::blockchain::Blockchain;

fn main() {
    // This is a placeholder for the command-line interface.
    // In a real application, we would parse command-line arguments
    // and load the configuration from a file.

    let config = MiningConfig {
        difficulty_target: 1000,
        target_triangle_depth: 10,
        mining_reward: 50,
        hardware: siertrichain::mining::config::HardwareSelection::Cpu,
    };

    let blockchain = Blockchain::new();
    let mut miner = Miner::new(config, blockchain);

    println!("Starting miner...");
    let block = miner.mine();
    println!("Mined new block: {:?}", block);
}
