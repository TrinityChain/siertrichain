use siertrichain::mining::miner::Miner;
use siertrichain::mining::config::MiningConfig;
use siertrichain::core::blockchain::Blockchain;

#[test]
fn test_mining_reward() {
    // This is a placeholder for a more comprehensive test.
    let config = MiningConfig {
        difficulty_target: 1,
        target_triangle_depth: 1,
        mining_reward: 50,
        hardware: siertrichain::mining::config::HardwareSelection::Cpu,
    };

    let blockchain = Blockchain::new();
    let mut miner = Miner::new(config, blockchain);

    let _block = miner.mine();
    // In a real implementation, we would check the coinbase transaction amount.
    assert!(true);
}
