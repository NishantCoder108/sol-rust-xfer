pub mod airdrop;
pub mod keygen;
pub mod transfer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        keygen::keygen();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
