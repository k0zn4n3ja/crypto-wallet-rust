use rand::prelude::*;
use rand::rngs::adapter::ReseedingRng;
use rand::rngs::OsRng;
use rand_chacha::ChaCha20Core;
use web3::types::U256;

pub fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

pub fn eth_to_wei(eth_val: f64) -> U256 {
    let result = eth_val * 1_000_000_000_000_000_000.0;
    let result = result as u128;

    U256::from(result)
}

/// Seeds a cryptographically secure random u64 using ChaCha20 CSPRING and OS randomness as seed.
///
/// The PNRG is reseeded from entropy every call.
///
/// Performance overhead may be excessive if seeding many PNRGs.
///
/// However, for the generation of a one-time private key for a wallet this function will give appropriate performance and security
///
pub fn random_seed() -> u64 {
    let prng = ChaCha20Core::from_entropy();
    let mut reseeding_rng = ReseedingRng::new(prng, 0, OsRng); //Reseeding
    reseeding_rng.gen::<u64>()
}
