use siertrichain::wallet::wallet::Wallet;
use siertrichain::wallet::address::Address;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about = "SierpinskiChain Wallet CLI")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new wallet
    New,
    /// Show wallet address (stub)
    Address,
    // You can add more commands here (e.g., send, balance)
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New => {
            let mut csprng = OsRng {};
            let keypair = Keypair::generate(&mut csprng);
            let wallet = Wallet::new(keypair);

            println!("New wallet created!");
            let address = Address::from_pubkey(&wallet.address());
            println!("Address: {}", address.to_base58());
            // You could also save the wallet to disk here
        }
        Commands::Address => {
            println!("Address command selected. (Not implemented)");
            // You could load a wallet and print its address here