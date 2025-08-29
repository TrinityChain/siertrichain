use siertrichain::wallet::wallet::Wallet;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use siertrichain::wallet::address::Address;

fn main() {
    // This is a placeholder for the command-line wallet application.
    // In a real application, we would use a proper command-line argument parser.

    let mut csprng = OsRng{};
    let keypair = Keypair::generate(&mut csprng);
    let wallet = Wallet::new(keypair);

    println!("New wallet created!");
    let address = Address::from_pubkey(&wallet.address());
    println!("Address: {}", address.to_base58());
}
