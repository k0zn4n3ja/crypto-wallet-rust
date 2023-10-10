use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::keccak256;
use web3::types::Address;
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

/// Generates a keypair using OS Rng.
/// For all major platforms, OS Rng is a CSPRNG with physical entropy as seed.
/// Secp256k1 is used by Bitcoin and Ethereum coin types
pub fn generate_keypair_secp256k1() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    secp.generate_keypair(&mut OsRng)
}
