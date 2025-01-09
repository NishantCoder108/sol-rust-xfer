use solana_sdk::signature::{Keypair, Signer};
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn generate_keypair(file_path: &str) -> Result<(), Box<dyn Error>> {
    let kp = Keypair::new();

    println!("Generated a new Solana wallet: {}", kp.pubkey().to_string());

    // Serialize the private key into JSON format
    let key_bytes = kp.to_bytes();
    println!("{:?}", kp.to_bytes());

    let mut file = File::create(file_path)?;
    file.write_all(&key_bytes)?;

    println!("Saved wallet to {}", file_path);

    Ok(())
}
