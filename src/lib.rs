pub mod airdrop;
pub mod keygen;
pub mod programs;
pub mod transfer;
pub mod utils;
use crate::programs::Turbin3_prereq::{CompleteArgs, UpdateArgs, WbaPrereqProgram};
use base64;
use dotenv::dotenv;
use solana_sdk::system_program;
use solana_sdk::{
    pubkey::Pubkey, signer::keypair::Keypair, signer::Signer, system_instruction,
    transaction::Transaction,
};

use bs58;
// use solana_sdk::signature::Keypair;
use std::env;

// use std::env;

use solana_client::rpc_client::RpcClient;

const RPC_URL: &str = "https://api.devnet.solana.com";

// /// Function to get the keypair from the `.env` file
// fn get_keypair_from_env() -> Keypair {
//     dotenv().ok(); // Load environment variables
//     let private_key_base64 = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env");

//     // Decode Base64 or JSON private key
//     let private_key_bytes: Vec<u8> = if private_key_base64.starts_with('[') {
//         // JSON format
//         serde_json::from_str(&private_key_base64).expect("Failed to parse PRIVATE_KEY")
//     } else {
//         // Base64 format
//         base64::decode(private_key_base64).expect("Failed to decode PRIVATE_KEY")
//     };

//     Keypair::from_bytes(&private_key_bytes).expect("Invalid private key")
// }

/// Function to get the keypair from the `.env` file
/// Function to get the keypair from the `.env` file
/// Function to get the keypair from the `.env` file
fn get_keypair_from_env() -> Keypair {
    dotenv().ok(); // Load environment variables

    // Retrieve the Base58-encoded private key
    let private_key_base58 = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env");

    // Decode Base58 private key into bytes
    let private_key_bytes = bs58::decode(private_key_base58)
        .into_vec()
        .expect("Failed to decode PRIVATE_KEY from Base58");

    // Ensure the decoded bytes are 64 bytes long
    assert_eq!(private_key_bytes.len(), 64, "Invalid private key length");

    // Create Keypair from the private key bytes
    Keypair::from_bytes(&private_key_bytes).expect("Invalid private key format")
}

/// Function to print the public key
pub fn print_public_key() {
    let keypair = get_keypair_from_env();
    println!("Public Key: {}", keypair.pubkey());
}

/// Function to deploy an on-chain program (or a placeholder transaction here)
pub fn deploy_on_chain() {
    // Constants
    const RPC_URL: &str = "https://api.devnet.solana.com";
    const GITHUB_USERNAME: &str = "NishantCoder108";

    // Initialize RPC client
    let rpc_client = RpcClient::new(RPC_URL);

    // Load signer from .env
    let signer = get_keypair_from_env();

    // Derive PDA (Program Derived Address)
    // let prereq_pda = WbaPrereqProgram::derive_program_address(&[
    //     b"prereq",
    //     signer.pubkey().to_bytes().as_ref(),
    // ]);

    let prereq =
        WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

    println!("Derived PDA: {}", prereq);

    // Prepare instruction arguments
    let args = CompleteArgs {
        github: GITHUB_USERNAME.as_bytes().to_vec(),
    };

    // Get the latest blockhash
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to fetch the recent blockhash");

    // Create the transaction
    let transaction = WbaPrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );

    // Send and confirm the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Transaction failed");

    // Output success message
    println!(
"Transaction successful! View it on Solana Explorer: https://explorer.solana.com/tx/{}/?cluster=devnet",
signature
);
}

/// Main entry function
pub fn main() {
    // Print public key for verification
    print_public_key();

    // Deploy or execute the on-chain logic
    deploy_on_chain();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_keygen() {
        let result = keygen::generate_keypair();
        assert!(
            result.is_ok(),
            "Failed to generate keypair: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_base58_to_wallet() {
        let base58 = "5HueCGU8rMjxEXxiPuD5BDuF43zoU3jFp1GWE2PaX4g8JFA6URg"; // Example
        let wallet = bs58::decode(base58).into_vec().unwrap();
        assert!(!wallet.is_empty());
    }

    #[test]
    fn test_wallet_to_base58() {
        let wallet: Vec<u8> = vec![34, 46, 55, 124, 141, 190, 24, 204, 134, 9];
        let base58 = bs58::encode(wallet).into_string();
        assert!(!base58.is_empty());
    }

    #[test]
    fn test_airdrop() {
        airdrop::airdrop();
    }

    #[test]
    fn test_transfer_sol() {
        transfer::transfer_sol();
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// pub fn main() {
//     println!("Hello from lib.rs!");
// }
