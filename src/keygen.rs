use solana_sdk::signature::{Keypair, Signer};
use std::error::Error;

pub fn generate_keypair() -> Result<(), Box<dyn Error>> {
    let kp = Keypair::new();

    println!("Generated a new Solana wallet: {}", kp.pubkey().to_string());

    println!("{:?}", kp.to_bytes());

    Ok(())
}
