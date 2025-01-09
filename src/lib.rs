pub mod airdrop;
pub mod keygen;
pub mod transfer;
pub mod utils;

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

pub fn main() {
    println!("Hello from lib.rs!");
}
