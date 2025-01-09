use bs58;
use std::io::{self, BufRead};

/// Converts a base58-encoded private key to a wallet file format (byte array).
pub fn base58_to_wallet() -> Result<(), Box<dyn std::error::Error>> {
    println!("Input your private key as base58:");

    // Read user input
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();

    // Decode base58 to a byte array
    match bs58::decode(&base58).into_vec() {
        Ok(wallet) => {
            println!("Your wallet file format (byte array) is:");
            println!("{:?}", wallet);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error decoding base58: {}", e);
            Err(Box::new(e))
        }
    }
}

/// Converts a wallet file format (byte array) to a base58-encoded private key.
pub fn wallet_to_base58() -> Result<(), Box<dyn std::error::Error>> {
    println!("Input your private key as a wallet file byte array (e.g., [34, 46, ...]):");

    // Read user input
    let stdin = io::stdin();
    let wallet_input = stdin.lock().lines().next().unwrap().unwrap();

    // Parse byte array from input
    let wallet = wallet_input
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>();

    // Encode byte array to base58
    match wallet {
        Ok(wallet_bytes) => {
            let base58 = bs58::encode(wallet_bytes).into_string();
            println!("Your private key in base58 format is:");
            println!("{}", base58);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error parsing wallet byte array: {}", e);
            Err(Box::new(e))
        }
    }
}
