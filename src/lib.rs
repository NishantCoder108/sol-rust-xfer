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
